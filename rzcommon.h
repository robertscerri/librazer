#ifndef RZCOMMON_H
#define RZCOMMON_H

#include <hidapi/hidapi.h>
#include <stdint.h>
#include <stdbool.h>

#define RZ_REPORT_LEN 90

#define RZ_REPORT_HDR_LEN 7

struct rz_report {
    uint8_t id;
    uint8_t cmd;
    uint8_t sub_cmd;
    uint8_t *params;
    unsigned int params_len;
};

uint8_t rz_calculate_crc(const uint8_t *data);
int rz_send_transfer(hid_device *dev, const struct rz_report *request);
bool rz_set_brightness(hid_device *dev, const float brightness);

#endif //RZCOMMON_H
