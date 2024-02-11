#include <stdio.h>
#include "hidtest.h"

const char *hid_bus_name(hid_bus_type bus_type) {
    static const char *const HidBusTypeName[] = {
        "Unknown",
        "USB",
        "Bluetooth",
        "I2C",
        "SPI",
    };

    if ((int)bus_type < 0)
        bus_type = HID_API_BUS_UNKNOWN;
    if ((int)bus_type >= (int)(sizeof(HidBusTypeName) / sizeof(HidBusTypeName[0])))
        bus_type = HID_API_BUS_UNKNOWN;

    return HidBusTypeName[bus_type];
}

void print_device(struct hid_device_info *cur_dev) {
    printf("Device Found\n  type: %04hx %04hx\n  path: %s\n  serial_number: %ls", cur_dev->vendor_id, cur_dev->product_id, cur_dev->path, cur_dev->serial_number);
    printf("\n");
    printf("  Manufacturer: %ls\n", cur_dev->manufacturer_string);
    printf("  Product:      %ls\n", cur_dev->product_string);
    printf("  Release:      %hx\n", cur_dev->release_number);
    printf("  Interface:    %d\n",  cur_dev->interface_number);
    printf("  Usage (page): 0x%hx (0x%hx)\n", cur_dev->usage, cur_dev->usage_page);
    printf("  Bus type: %u (%s)\n", (unsigned)cur_dev->bus_type, hid_bus_name(cur_dev->bus_type));
    printf("\n");
}

void print_hid_report_descriptor_from_device(hid_device *device) {
    unsigned char descriptor[HID_API_MAX_REPORT_DESCRIPTOR_SIZE];
    int res = 0;

    printf("  Report Descriptor: ");
    res = hid_get_report_descriptor(device, descriptor, sizeof(descriptor));
    if (res < 0) {
        printf("error getting: %ls", hid_error(device));
    }
    else {
        printf("(%d bytes)", res);
    }
    for (int i = 0; i < res; i++) {
        if (i % 10 == 0) {
            printf("\n");
        }
        printf("0x%02x, ", descriptor[i]);
    }
    printf("\n");
}