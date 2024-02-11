#ifndef RZCOMMON_H
#define RZCOMMON_H

#include <libusb/libusb.h>
#include <stdint.h>
#include <stdbool.h>

#define RZ_REPORT_LEN 90

#define RZ_REPORT_HDR_LEN 7

struct rz_report {
    unsigned char id;
    unsigned char cmd;
    unsigned char sub_cmd;
    unsigned char *params;
    unsigned int params_len;
};

uint8_t rz_calculate_crc(const uint8_t *data);
int rz_send_transfer(libusb_device_handle *dev, const struct rz_report *request);
bool rz_set_brightness(libusb_device_handle *dev, const float brightness);

#endif //RZCOMMON_H
