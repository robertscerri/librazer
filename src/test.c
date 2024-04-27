#include <stdio.h>
#include <libusb-1.0/libusb.h>
#include <unistd.h>

#include "rzcommon.h"
#include "chromacommon.h"

void test_matrix(const struct rz_device *dev);

int main(int argc, char **argv) {
    libusb_context* context = NULL;
    libusb_init_context(&context, NULL, 0);

    if (argc < 2) {
        fprintf(stderr, "Please specify a product ID\n");
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

    test_matrix(&dev);

    rz_close_device(dev);

    libusb_exit(context);
}

void test_matrix(const struct rz_device *dev) {
    const struct rz_rgb red = {0xff, 0, 0};
    const struct rz_rgb green = {0, 0xff, 0};
    const struct rz_rgb blue = {0, 0, 0xff};
    struct rz_rgb rowValues[9] = {red, red, red, green, green, green, blue, blue, blue};

    struct rz_rgb_row row;
    row.start = 1;
    row.end = 9;
    row.rgb_values = rowValues;

    struct rz_rgb_row rows[1] = {row};

    struct rz_rgb_matrix testMatrix;
    testMatrix.row_count = 1;
    testMatrix.rows = rows;

    rz_set_effect_custom(dev, &testMatrix);
}