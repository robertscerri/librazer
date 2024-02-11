#include "rzcommon.h"

#include <stdio.h>

#include "rzdevices.h"
#include "usbcommon.h"

static uint8_t rz_calculate_crc(const uint8_t *data) {
    uint8_t crc = 0;

    for (int i = 2; i < 88; i++) {
        crc ^= data[i];
    }

    return crc;
}

static void rz_data_from_report(const struct rz_report *report, uint8_t *data) {
    unsigned int data_offset = 0;

    const uint8_t NUM_PARAMS = report->params_len + 1; //params_len + 1 for sub_cmd
    const uint8_t DATA_HDR[7] = {0x00, report->id, 0x00,  0x00, 0x00, NUM_PARAMS, 0x03}; //Middle 0x00 and last 0x03 reserved byte

    const uint8_t DATA_CMD[2] = {report->cmd, report->sub_cmd};

    memcpy(data + data_offset, DATA_HDR, 7);
    data_offset += 7;

    memcpy(data + data_offset, DATA_CMD, 2);
    data_offset += 2;

    memcpy(data + data_offset, report->params, report->params_len);
    data_offset += report->params_len;

    while (data_offset < RZ_REPORT_LEN) {
        data[data_offset] = 0;
        data_offset++;
    }

    data[RZ_REPORT_LEN - 2] = rz_calculate_crc(data);
}

static void clampf(float *val, uint8_t min, uint8_t max) {
    if (*val > max) {
        *val = max;
    } else if (*val < min) {
        *val = min;
    }
}

int rz_send_report(const struct rz_device *dev, const struct rz_report *report) {
    uint8_t data[RZ_REPORT_LEN];

    rz_data_from_report(report, data);

    int ctrl = libusb_control_transfer(
        dev->usb_dev,
        0x21,
        0x09,
        0x0300,
        dev->w_index,
        data,
        RZ_REPORT_LEN,
        2000);

    return ctrl;
}

bool rz_set_brightness(const struct rz_device *dev, float brightness) {
    clampf(&brightness, 0, 1);
    uint8_t PARAMS[2] = {0x05, (brightness * 255)};

    struct rz_report report;
    report.id = 0x1f;
    report.cmd = 0x03;
    report.sub_cmd = 0x01;
    report.params = PARAMS;
    report.params_len = 2;

    return rz_send_report(dev, &report) > 0;
}

static uint16_t rz_get_w_index(const uint16_t pid) {
    switch(pid) {
        case RZ_PID_BLACKWIDOW_CHROMA_V2:
            return 0x02;
        default:
            return 0x00;
    }
}

struct rz_device rz_open_device(const uint16_t pid) {
    struct rz_device dev;

    dev.pid = pid;
    dev.usb_dev = usb_get_device_by_pid(pid);
    dev.w_index = rz_get_w_index(pid);

    // Claim the interface specified by wIndex
    if (libusb_claim_interface(dev.usb_dev, dev.w_index) < 0) {
        fprintf(stderr, "Failed to claim interface\n");
        libusb_close(dev.usb_dev);
        libusb_exit(NULL);
        return dev;
    }

    return dev;
}

void rz_close_device(struct rz_device dev) {
    libusb_release_interface(dev.usb_dev, dev.w_index);
    libusb_close(dev.usb_dev);
}