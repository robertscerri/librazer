#include <stdio.h>
#include <hidapi/hidapi_winapi.h>
#include <unistd.h>

#include "rzcommon.h"

#define MAX_STR 255

void constructDataFragment(uint8_t *data, uint8_t id, uint8_t cmd, uint8_t sub_cmd, const uint8_t *params, unsigned int params_length);
uint8_t calculateCRC(const uint8_t *data);

int main(int argc, char* argv[]) {
    int res;
    wchar_t wstr[MAX_STR];

    res = hid_init();

    hid_device *mouse = hid_open(0x1532, 0x0046, NULL);
    if (!mouse) {
        printf("Unable to open device\n");
        hid_exit();
        return 1;
    }

    hid_device *mousemat = hid_open(0x1532, 0x0c00, NULL);
    if (!mousemat) {
        printf("Unable to open device\n");
        hid_exit();
        return 1;
    }

    hid_device *kbd = hid_open(0x1532, 0x0221, NULL);
    if (!kbd) {
        printf("Unable to open device\n");
        hid_exit();
        return 1;
    }

    wchar_t p[255];
    hid_get_product_string(kbd, p, 255);
    printf("%ls", p);

    // rz_set_brightness(mouse, 0.3f);
    // //sleep(1);
    //rz_set_brightness(mouse, 1.0f);
    // //sleep(1);
    // rz_set_brightness(mousemat, 0.3f);
    // //sleep(1);
    // rz_set_brightness(mousemat, 1.0f);
    //sleep(1);
    rz_set_brightness(kbd, 0.3f);
    //sleep(1);
    //rz_set_brightness(kbd, 1.0f);
    //sleep(1);
}