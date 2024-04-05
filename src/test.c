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
    const struct rz_rgb black = {0x00, 0x00, 0x00};

//    rz_set_colour_kbd(&dev, green);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, green, 3, 6);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, green, 3, 6);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, blue, 6, 9);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, cyan, 9, 12);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, magenta, 12, 13);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, yellow, 14, 14);
//    usleep(100000);
//    rz_set_effect_custom_frame(&dev, white, 15, 15);

//    rz_set_colour(&dev, red);
//    usleep(100000);
//    rz_set_colour(&dev, green);
//    usleep(100000);
//    rz_set_colour(&dev, blue);
//    usleep(100000);
//    rz_set_colour(&dev, cyan);
//    usleep(100000);
//    rz_set_colour(&dev, magenta);
//    usleep(100000);
//    rz_set_colour_kbd(&dev, yellow);
//    usleep(100000);
//    rz_set_colour_kbd(&dev, white);
//    sleep(2);
//    rz_set_effect_wave(&dev, RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT);
//    sleep(2);
//    rz_set_effect_wave(&dev, RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT);
//    sleep(2);
//    rz_set_effect_spectrum(&dev);

    rz_close_device(dev);

    libusb_exit(context);
}
