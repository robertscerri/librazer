#ifndef RZCOMMON_H
#define RZCOMMON_H

#include <libusb/libusb.h>
#include <stdint.h>
#include <stdbool.h>

#define RZ_VENDOR_ID 0x1532

#define RZ_REPORT_LEN 90

struct rz_device {
    libusb_device_handle *usb_dev;
    unsigned int pid;
    unsigned int wIndex;
};

struct rz_report {
    uint8_t id;
    uint8_t cmd;
    uint8_t sub_cmd;
    uint8_t *params;
    unsigned int params_len;
};

int rz_send_transfer(libusb_device_handle *dev, const struct rz_report *request);
bool rz_set_brightness(libusb_device_handle *dev, const float brightness);

#endif //RZCOMMON_H
