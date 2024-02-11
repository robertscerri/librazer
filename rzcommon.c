#include "rzcommon.h"

static void rz_data_from_report(const struct rz_report *report, unsigned char *data) {
    unsigned int data_offset = 0;

    const unsigned char NUM_PARAMS = report->params_len + 1; //params_len + 1 for sub_cmd
    const unsigned char DATA_HDR[7] = {0x00, report->id, 0x00,  0x00, 0x00, NUM_PARAMS, 0x03}; //Middle 0x00 and last 0x03 reserved byte

    const unsigned char DATA_CMD[2] = {report->cmd, report->sub_cmd};

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

unsigned char rz_calculate_crc(const unsigned char *data) {
    unsigned char crc = 0;

    for (int i = 2; i < 88; i++) {
        crc ^= data[i];
    }

    return crc;
}

int rz_send_report(libusb_device_handle *dev, const struct rz_report *report) {
    unsigned char data[RZ_REPORT_LEN];

    rz_data_from_report(report, data);

    int ctrl = libusb_control_transfer(
        dev,
        0x21,
        0x09,
        0x0300,
        0x02,
        data,
        RZ_REPORT_LEN,
        2000);

    return ctrl;
}

bool rz_set_brightness(libusb_device_handle *dev, const float brightness) {
    unsigned char PARAMS[2] = {0x05, (brightness * 255)};

    struct rz_report report;
    report.id = 0x1f;
    report.cmd = 0x03;
    report.sub_cmd = 0x01;
    report.params = PARAMS;
    report.params_len = 2;

    return rz_send_report(dev, &report) > 0;
}