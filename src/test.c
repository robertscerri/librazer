#include <stdio.h>
#include <libusb/libusb.h>
#include <unistd.h>

#include "rzcommon.h"
#include "chromacommon.h"

int main(int argc, char **argv) {
    libusb_context* context = NULL;
    libusb_init_context(&context, NULL, 0);

    if (argc < 2) {
        printf("Please specify a product ID");
        exit(1);
    }

    struct rz_device dev = rz_open_device(strtol(argv[1], NULL, 0));

//    rz_set_brightness(&dev, 0.3f);
//    sleep(1);
//    rz_set_brightness(&dev, 0);
//    sleep(1);
//    rz_set_brightness(&dev, 1.0f);
//    sleep(1);

    const struct rz_rgb red = {0xff, 0, 0};
    const struct rz_rgb green = {0, 0xff, 0};
    const struct rz_rgb blue = {0, 0, 0xff};
    const struct rz_rgb cyan = {0, 0xff, 0xff};
    const struct rz_rgb magenta = {0xff, 0, 0xff};
    const struct rz_rgb yellow = {0xff, 0xff, 0};
    const struct rz_rgb white = {0xff, 0xff, 0xff};

    rz_set_colour(&dev, red);
    usleep(10000);
    rz_set_colour(&dev, green);
    usleep(10000);
    rz_set_colour(&dev, blue);
    usleep(10000);
    rz_set_colour(&dev, cyan);
    usleep(10000);
    rz_set_colour(&dev, magenta);
    usleep(10000);
    rz_set_colour(&dev, yellow);
    usleep(10000);
    rz_set_colour(&dev, white);
    sleep(2);
    rz_set_effect_wave(&dev, RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT);
    sleep(2);
    rz_set_effect_wave(&dev, RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT);
    sleep(2);
    rz_set_effect_spectrum(&dev);

    rz_close_device(dev);

    libusb_exit(context);
}
