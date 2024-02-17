#include "usbcommon.h"

#include <stdio.h>

#include "rzcommon.h"

libusb_device_handle *usb_get_device_by_pid(const uint16_t pid) {
    libusb_device **devices;
    libusb_device *dev;
    libusb_device_handle *dev_handle = NULL;

    size_t i = 0;
    int res;

    if (libusb_get_device_list(NULL, &devices) < 0) {
        return NULL;
    }

    while ((dev = devices[i++]) != NULL) {
        struct libusb_device_descriptor desc;
        res = libusb_get_device_descriptor(dev, &desc);

        if (res == 0) {
            if (desc.idVendor == RZ_VENDOR_ID && desc.idProduct == pid) {
                res = libusb_open(dev, &dev_handle);

                if (res < 0) {
                    dev_handle = NULL;
                }

                break;
            }
        } else {
            break;
        }
    }

    libusb_free_device_list(devices, 1);
    return dev_handle;
}
