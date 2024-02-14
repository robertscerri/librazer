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

    rz_set_effect_spectrum(&mouse);
    rz_set_effect_spectrum(&mousemat);
    rz_set_effect_spectrum(&kbd);

    rz_close_device(mouse);
    rz_close_device(mousemat);
    rz_close_device(kbd);

    libusb_exit(context);
}