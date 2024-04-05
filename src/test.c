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

    const struct rz_rgb red = {0xff, 0, 0};
    const struct rz_rgb green = {0, 0xff, 0};
    const struct rz_rgb blue = {0, 0, 0xff};
    const struct rz_rgb cyan = {0, 0xff, 0xff};
    const struct rz_rgb magenta = {0xff, 0, 0xff};
    const struct rz_rgb yellow = {0xff, 0xff, 0};
    const struct rz_rgb white = {0xff, 0xff, 0xff};
    const struct rz_rgb black = {0x00, 0x00, 0x00};

    struct rz_rgb testRowValues[16];
    for (int i = 0; i < 16; i++) {
        testRowValues[i] = black;
    }
    struct rz_rgb_matrix_row clearRow = {0, 15, testRowValues};

    struct rz_rgb row3Values[5] = {cyan, magenta, yellow, red, green};
    struct rz_rgb_matrix_row testRow;
    testRow.start = 5;
    testRow.end = 9;
    testRow.rgb_values = row3Values;

    struct rz_rgb_matrix_row rows[1] = {testRow};

    struct rz_rgb_matrix testMatrix;
    testMatrix.row_count = 1;
    testMatrix.rows = rows;

    rz_set_effect_custom(&dev, &testMatrix);

    rz_close_device(dev);

    libusb_exit(context);
}
