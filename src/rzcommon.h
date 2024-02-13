#ifndef RZCOMMON_H
#define RZCOMMON_H

#include <libusb/libusb.h>
#include <stdint.h>
#include <stdbool.h>

#define RZ_VENDOR_ID 0x1532

#define RZ_REPORT_LEN 90

struct rz_device {
    libusb_device_handle *usb_dev;
    uint16_t pid;
    uint16_t w_index;
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

#define RZ_CHROMA_EFFECT_NONE 0x00
#define RZ_CHROMA_EFFECT_WAVE 0x01
#define RZ_CHROMA_EFFECT_REACTIVE 0x02
#define RZ_CHROMA_EFFECT_BREATH 0x03
#define RZ_CHROMA_EFFECT_SPECTRUM 0x04
#define RZ_CHROMA_EFFECT_CUSTOM 0x05 // draw frame
#define RZ_CHROMA_EFFECT_STATIC 0x06
#define RZ_CHROMA_EFFECT_CLEAR_ROW 0x08

bool rz_set_brightness(const struct rz_device *dev, float brightness);
bool rz_set_effect(const struct rz_device *dev, const unsigned int effect_id, const uint8_t params, const unsigned int params_len);
bool rz_set_effect_static(const struct rz_device *dev, float r, float g, float b);

#endif //RZCOMMON_H
