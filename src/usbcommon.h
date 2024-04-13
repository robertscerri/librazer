#ifndef USBCOMMON_H
#define USBCOMMON_H

#include <libusb-1.0/libusb.h>
#include <stdint.h>

libusb_device_handle *usb_get_device_by_pid(const uint16_t pid);

#endif //USBCOMMON_H
