#include "rzcommon.h"

static void rz_data_from_report(const struct rz_report *report, uint8_t *data) {
    uint8_t *data_start = data;

    *data = 0x00; //Report ID
    data++;

    const uint8_t NUM_PARAMS = report->params_len + 1; //params_len + 1 for sub_cmd
    const uint8_t DATA_HDR[7] = {0x00, report->id, 0x00,  0x00, 0x00, NUM_PARAMS, 0x03}; //Last 0x03 reserved byte

    const uint8_t DATA_CMD[2] = {report->cmd, report->sub_cmd};

    memcpy(data, DATA_HDR, 7);
    data += 7;

    memcpy(data, DATA_CMD, 2);
    data += 2;

    memcpy(data, report->params, report->params_len);
    data += report->params_len;

    while (data != data_start + ((RZ_REPORT_LEN + 1) * sizeof(uint8_t))) {
        *data = 0;
        data++;
    }

    data_start[RZ_REPORT_LEN + 1 - 2] = rz_calculate_crc(data_start);
}

uint8_t rz_calculate_crc(const uint8_t *data) {
    uint8_t crc = 0;
    data++; //Skip report ID

    for (int i = 2; i < 88; i++) {
        crc ^= data[i];
    }

    return crc;
}

int rz_send_report(hid_device *dev, const struct rz_report *report) {
    uint8_t data[RZ_REPORT_LEN + 1];

    rz_data_from_report(report, data);

    return hid_send_feature_report(dev, data, RZ_REPORT_LEN + 1);
}

bool rz_set_brightness(hid_device *dev, const float brightness) {
    uint8_t PARAMS[2] = {0x05, (brightness * 255)};

    struct rz_report report;
    report.id = 0x1f;
    report.cmd = 0x03;
    report.sub_cmd = 0x01;
    report.params = PARAMS;
    report.params_len = 2;

    return rz_send_transfer(dev, &report) != -1;
}