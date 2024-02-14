#include <stdio.h>
#include <libusb/libusb.h>
#include <unistd.h>

#include "rzcommon.h"
#include "chromacommon.h"

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

    struct rz_rgb red = {1.0f, 0, 0};
    struct rz_rgb green = {0, 1.0f, 0};
    struct rz_rgb blue = {0, 0, 1.0f};

    rz_set_effect_static(&mouse, red);
    rz_set_effect_static(&mousemat, red);
    rz_set_effect_static(&kbd, red);
    sleep(4);
    rz_set_effect_static(&mouse, green);
    rz_set_effect_static(&mousemat, green);
    rz_set_effect_static(&kbd, green);
    sleep(4);
    rz_set_effect_static(&mouse, blue);
    rz_set_effect_static(&mousemat, blue);
    rz_set_effect_static(&kbd, blue);

    rz_close_device(mouse);
    rz_close_device(mousemat);
    rz_close_device(kbd);

    libusb_exit(context);
}