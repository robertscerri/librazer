#ifndef RZCOMMON_H
#define RZCOMMON_H

#include <libusb-1.0/libusb.h>
#include <stdint.h>

#define RZ_VENDOR_ID 0x1532

#define RZ_REPORT_LEN 90

enum rz_device_type {
    MOUSE,
    KEYBOARD,
    KRAKEN,
    ACCESSORY
};

struct rz_device {
    libusb_device_handle *usb_dev;
    uint16_t pid;
    uint16_t w_index;
    enum rz_device_type type;
};

struct rz_report {
    uint8_t id;
    uint8_t cmd;
    uint8_t sub_cmd;
    uint8_t *params;
    unsigned int params_len;
};

int rz_send_report(const struct rz_device *dev, const struct rz_report *report);
struct rz_device rz_open_device(uint16_t pid);
void rz_close_device(struct rz_device dev);

#endif //RZCOMMON_H
