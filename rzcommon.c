#include "rzcommon.h"

#include <stdio.h>

static void rz_data_from_report(const struct rz_report *report, unsigned char *data) {
    unsigned char *data_start = data;

    const unsigned char NUM_PARAMS = report->params_len + 1; //params_len + 1 for sub_cmd
    const unsigned char DATA_HDR[7] = {0x00, report->id, 0x00,  0x00, 0x00, NUM_PARAMS, 0x03}; //Last 0x03 reserved byte

    const unsigned char DATA_CMD[2] = {report->cmd, report->sub_cmd};

    memcpy(data, DATA_HDR, 7);
    data += 7;

    memcpy(data, DATA_CMD, 2);
    data += 2;

    memcpy(data, report->params, report->params_len);
    data += report->params_len;

    while (data != data_start + ((RZ_REPORT_LEN + 1) * sizeof(char))) {
        *data = 0;
        data++;
    }

    data_start[RZ_REPORT_LEN - 2] = rz_calculate_crc(data_start);
}

unsigned char rz_calculate_crc(const uint8_t *data) {
    unsigned char crc = 0;

    for (int i = 2; i < 88; i++) {
        crc ^= data[i];
    }

    return crc;
}

int rz_send_report(libusb_device_handle *dev, const struct rz_report *report) {
    unsigned char * data[RZ_REPORT_LEN + 1];

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

    if (ctrl == LIBUSB_ERROR_TIMEOUT) {
        printf("Connection timed out\n");
    }
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