#ifndef HIDTEST_H
#define HIDTEST_H
#include <hidapi/hidapi.h>

const char *hid_bus_name(hid_bus_type bus_type);
void print_device(struct hid_device_info *cur_dev);
void print_hid_report_descriptor_from_device(hid_device *device);

#endif //HIDTEST_H
