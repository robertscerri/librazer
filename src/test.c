#include <stdio.h>
#include <libusb/libusb.h>
#include <unistd.h>

#include "rzcommon.h"

int main(void) {
    libusb_context* context = NULL;
    libusb_init_context(&context, NULL, 0);

    struct rz_device mouse = rz_open_device(0x0046);
    struct rz_device mousemat = rz_open_device(0x0c00);
    struct rz_device kbd = rz_open_device(0x0221);

    rz_set_brightness(&mouse, 0.3f);
    rz_set_brightness(&mousemat, 0.3f);
    rz_set_brightness(&kbd, 0.3f);
    sleep(1);
    rz_set_brightness(&mouse, 1.0f);
    rz_set_brightness(&mousemat, 1.0f);
    rz_set_brightness(&kbd, 1.0f);
    sleep(1);
    rz_set_effect_static(&mouse, 255, 0, 0);
    rz_set_effect_static(&mousemat, 255, 0, 0);
    rz_set_effect_static(&kbd, 255, 0, 0);
    sleep(4);
    rz_set_effect_static(&mouse, 0, 255, 0);
    rz_set_effect_static(&mousemat, 0, 255, 0);
    rz_set_effect_static(&kbd, 0, 255, 0);
    sleep(4);
    rz_set_effect_static(&mouse, 0, 0, 255);
    rz_set_effect_static(&mousemat, 0, 0, 255);
    rz_set_effect_static(&kbd, 0, 0, 255);
    sleep(4);
    rz_set_effect(&mouse, RZ_CHROMA_EFFECT_SPECTRUM, NULL, 0);
    rz_set_effect(&mousemat, RZ_CHROMA_EFFECT_SPECTRUM, NULL, 0);
    rz_set_effect(&kbd, RZ_CHROMA_EFFECT_SPECTRUM, NULL, 0);

    rz_close_device(mouse);
    rz_close_device(mousemat);
    rz_close_device(kbd);

    libusb_exit(context);
}