#include <stdio.h>
#include <libusb/libusb.h>
#include <unistd.h>

#include "rzcommon.h"

void constructDataFragment(uint8_t *data, uint8_t id, uint8_t cmd, uint8_t sub_cmd, const uint8_t *params, unsigned int params_length);
uint8_t calculateCRC(const uint8_t *data);

int main(int argc, char* argv[]) {
    libusb_context* context = NULL;
    libusb_init_context(&context, NULL, 0);

    libusb_device_handle *mouse = libusb_open_device_with_vid_pid(NULL, 0x1532, 0x0046);
    if (!mouse) {
        printf("Unable to open device\n");
        return 1;
    }

    libusb_device_handle *mousemat = libusb_open_device_with_vid_pid(NULL, 0x1532, 0x0c00);
    if (!mousemat) {
        printf("Unable to open device\n");
        return 1;
    }

    libusb_device_handle *kbd = libusb_open_device_with_vid_pid(NULL, 0x1532, 0x0221);
    if (!kbd) {
        printf("Unable to open device\n");
        return 1;
    }

    // Claim the interface before performing any communication
    if (libusb_claim_interface(kbd, 2) < 0) {
        fprintf(stderr, "Failed to claim interface\n");
        libusb_close(kbd);
        libusb_exit(context);
        return 1;
    }

    rz_set_brightness(mouse, 0.3f);
    sleep(1);
    rz_set_brightness(mouse, 1.0f);
    sleep(1);
    rz_set_brightness(mousemat, 0.3f);
    sleep(1);
    rz_set_brightness(mousemat, 1.0f);
    sleep(1);
    rz_set_brightness(kbd, 0.3f);
    sleep(1);
    rz_set_brightness(kbd, 1.0f);
    sleep(1);

    libusb_release_interface(kbd, 2);

    libusb_close(mouse);
    libusb_close(mousemat);
    libusb_close(kbd);

    libusb_exit(context);
}