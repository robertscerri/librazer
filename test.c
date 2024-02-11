#include <stdio.h>
#include <hidapi/hidapi_winapi.h>

#define MAX_STR 255

int main(int argc, char* argv[]) {
    int res;
    unsigned char buf[65];
    wchar_t wstr[MAX_STR];

    res = hid_init();

    hid_device *handle;
    handle = hid_open(0x1532, 0x0046, NULL);
    if (!handle)
    {
        printf("Unable to open device\n");
        hid_exit();
        return 1;
    }

    // Read the Manufacturer String
    res = hid_get_manufacturer_string(handle, wstr, MAX_STR);
    printf("Manufacturer String: %ls\n", wstr);

    // Read the Product String
    res = hid_get_product_string(handle, wstr, MAX_STR);
    printf("Product String: %ls\n", wstr);
}