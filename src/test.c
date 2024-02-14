#include <stdio.h>
#include <libusb/libusb.h>
#include <unistd.h>

#include "rzcommon.h"
#include "chromacommon.h"
#include "rzdevices.h"

int main(void) {
    libusb_context* context = NULL;
    libusb_init_context(&context, NULL, 0);

    struct rz_device mouse = rz_open_device(RZ_PID_MAMBA_TE_WIRED);
    struct rz_device mousemat = rz_open_device(RZ_PID_FIREFLY);
    struct rz_device kbd = rz_open_device(RZ_PID_BLACKWIDOW_CHROMA_V2);

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
