#ifndef RZCOMMON_H
#define RZCOMMON_H

#include <libusb/libusb.h>
#include <stdbool.h>

#define RZ_VENDOR_ID 0x1532

#define RZ_REPORT_LEN 90

struct rz_report {
    unsigned char id;
    unsigned char cmd;
    unsigned char sub_cmd;
    unsigned char *params;
    unsigned int params_len;
};

int rz_send_transfer(libusb_device_handle *dev, const struct rz_report *request);
bool rz_set_brightness(libusb_device_handle *dev, const float brightness);

#endif //RZCOMMON_H
