#include "rzcommon.h"

#include <stdio.h>

#include "rzdevices.h"
#include "usbcommon.h"

static uint8_t rz_calculate_crc(const uint8_t *data) {
    uint8_t crc = 0;

    for (int i = 2; i < 88; i++) {
        crc ^= data[i];
    }

    return crc;
}

static void rz_data_from_report(const struct rz_report *report, uint8_t *data) {
    unsigned int data_offset = 0;

    const uint8_t NUM_PARAMS = report->params_len + 1; //params_len + 1 for sub_cmd
    const uint8_t DATA_HDR[7] = {0x00, report->id, 0x00,  0x00, 0x00, NUM_PARAMS, 0x03}; //Middle 0x00 and last 0x03 reserved byte

    const uint8_t DATA_CMD[2] = {report->cmd, report->sub_cmd};

    memcpy(data + data_offset, DATA_HDR, 7);
    data_offset += 7;

    memcpy(data + data_offset, DATA_CMD, 2);
    data_offset += 2;

    memcpy(data + data_offset, report->params, report->params_len);
    data_offset += report->params_len;

    while (data_offset < RZ_REPORT_LEN) {
        data[data_offset] = 0;
        data_offset++;
    }

    data[RZ_REPORT_LEN - 2] = rz_calculate_crc(data);
}

int rz_send_report(const struct rz_device *dev, const struct rz_report *report) {
    uint8_t data[RZ_REPORT_LEN];

    rz_data_from_report(report, data);

    int ctrl = libusb_control_transfer(
        dev->usb_dev,
        0x21,
        0x09,
        0x0300,
        dev->w_index,
        data,
        RZ_REPORT_LEN,
        2000);

    return ctrl;
}

static enum rz_device_type rz_get_device_type(const uint16_t pid) {
    switch(pid) {
        case RZ_PID_IRICHU_2011:
        case RZ_PID_DEATHADDER_3_5G:
        case RZ_PID_ABYSSUS_1800:
        case RZ_PID_MAMBA_2012_WIRED:
        case RZ_PID_MAMBA_2012_WIRELESS:
        case RZ_PID_DEATHADDER_3_5G_BLACK:
        case RZ_PID_NAGA_2012:
        case RZ_PID_IMPERATOR:
        case RZ_PID_OUROBOROS:
        case RZ_PID_TAIPAN:
        case RZ_PID_NAGA_HEX_RED:
        case RZ_PID_DEATHADDER_2013:
        case RZ_PID_DEATHADDER_1800:
        case RZ_PID_OROCHI_2013:
        case RZ_PID_NAGA_EPIC_CHROMA:
        case RZ_PID_NAGA_EPIC_CHROMA_DOCK:
        case RZ_PID_NAGA_2014:
        case RZ_PID_NAGA_HEX:
        case RZ_PID_ABYSSUS:
        case RZ_PID_DEATHADDER_CHROMA:
        case RZ_PID_MAMBA_WIRED:
        case RZ_PID_MAMBA_WIRELESS:
        case RZ_PID_MAMBA_TE_WIRED:
        case RZ_PID_OROCHI_CHROMA:
        case RZ_PID_DIAMONDBACK_CHROMA:
        case RZ_PID_DEATHADDER_2000:
        case RZ_PID_NAGA_HEX_V2:
        case RZ_PID_NAGA_CHROMA:
        case RZ_PID_DEATHADDER_3500:
        case RZ_PID_LANCEHEAD_WIRED:
        case RZ_PID_LANCEHEAD_WIRELESS:
        case RZ_PID_ABYSSUS_V2:
        case RZ_PID_DEATHADDER_ELITE:
        case RZ_PID_ABYSSUS_2000:
        case RZ_PID_LANCEHEAD_TE_WIRED:
        case RZ_PID_ATHERIS_RECEIVER:
        case RZ_PID_BASILISK:
        case RZ_PID_BASILISK_ESSENTIAL:
        case RZ_PID_NAGA_TRINITY:
        case RZ_PID_ABYSSUS_ELITE_DVA_EDITION:
        case RZ_PID_ABYSSUS_ESSENTIAL:
        case RZ_PID_MAMBA_ELITE:
        case RZ_PID_DEATHADDER_ESSENTIAL:
        case RZ_PID_LANCEHEAD_WIRELESS_RECEIVER:
        case RZ_PID_LANCEHEAD_WIRELESS_WIRED:
        case RZ_PID_DEATHADDER_ESSENTIAL_WHITE_EDITION:
        case RZ_PID_MAMBA_WIRELESS_RECEIVER:
        case RZ_PID_MAMBA_WIRELESS_WIRED:
        case RZ_PID_PRO_CLICK_RECEIVER:
        case RZ_PID_VIPER:
        case RZ_PID_VIPER_ULTIMATE_WIRED:
        case RZ_PID_VIPER_ULTIMATE_WIRELESS:
        case RZ_PID_DEATHADDER_V2_PRO_WIRED:
        case RZ_PID_DEATHADDER_V2_PRO_WIRELESS:
        case RZ_PID_PRO_CLICK_WIRED:
        case RZ_PID_BASILISK_X_HYPERSPEED:
        case RZ_PID_DEATHADDER_V2:
        case RZ_PID_BASILISK_V2:
        case RZ_PID_BASILISK_ULTIMATE_WIRED:
        case RZ_PID_BASILISK_ULTIMATE_RECEIVER:
        case RZ_PID_VIPER_MINI:
        case RZ_PID_DEATHADDER_V2_MINI:
        case RZ_PID_NAGA_LEFT_HANDED_2020:
        case RZ_PID_NAGA_PRO_WIRED:
        case RZ_PID_NAGA_PRO_WIRELESS:
        case RZ_PID_VIPER_8K:
        case RZ_PID_OROCHI_V2_RECEIVER:
        case RZ_PID_OROCHI_V2_BLUETOOTH:
        case RZ_PID_NAGA_X:
        case RZ_PID_DEATHADDER_ESSENTIAL_2021:
        case RZ_PID_BASILISK_V3:
        case RZ_PID_PRO_CLICK_MINI_RECEIVER:
        case RZ_PID_DEATHADDER_V2_X_HYPERSPEED:
        case RZ_PID_VIPER_MINI_SE_WIRED:
        case RZ_PID_VIPER_MINI_SE_WIRELESS:
        case RZ_PID_DEATHADDER_V2_LITE:
        case RZ_PID_COBRA:
        case RZ_PID_VIPER_V2_PRO_WIRED:
        case RZ_PID_VIPER_V2_PRO_WIRELESS:
        case RZ_PID_BASILISK_V3_PRO_WIRED:
        case RZ_PID_BASILISK_V3_PRO_WIRELESS:
        case RZ_PID_COBRA_PRO:
        case RZ_PID_DEATHADDER_V3:
        case RZ_PID_HYPERPOLLING_WIRELESS_DONGLE:
        case RZ_PID_NAGA_V2_HYPERSPEED_RECEIVER:
        case RZ_PID_DEATHADDER_V3_PRO_WIRED:
        case RZ_PID_DEATHADDER_V3_PRO_WIRELESS:
        case RZ_PID_VIPER_V3_HYPERSPEED:
            return MOUSE;
        case RZ_PID_BLACKWIDOW_ULTIMATE_2012:
        case RZ_PID_BLACKWIDOW_STEALTH_EDITION:
        case RZ_PID_ANANSI:
        case RZ_PID_NOSTROMO:
        case RZ_PID_ORBWEAVER:
        case RZ_PID_DEATHSTALKER_ESSENTIAL:
        case RZ_PID_BLACKWIDOW_ULTIMATE_2013:
        case RZ_PID_BLACKWIDOW_STEALTH:
        case RZ_PID_BLACKWIDOW_TE_2014:
        case RZ_PID_TARTARUS:
        case RZ_PID_DEATHSTALKER_EXPERT:
        case RZ_PID_BLACKWIDOW_CHROMA:
        case RZ_PID_DEATHSTALKER_CHROMA:
        case RZ_PID_BLADE_STEALTH:
        case RZ_PID_ORBWEAVER_CHROMA:
        case RZ_PID_TARTARUS_CHROMA:
        case RZ_PID_BLACKWIDOW_CHROMA_TE:
        case RZ_PID_BLADE_QHD:
        case RZ_PID_BLADE_PRO_LATE_2016:
        case RZ_PID_BLACKWIDOW_OVERWATCH:
        case RZ_PID_BLACKWIDOW_ULTIMATE_2016:
        case RZ_PID_BLACKWIDOW_X_CHROMA:
        case RZ_PID_BLACKWIDOW_X_ULTIMATE:
        case RZ_PID_BLACKWIDOW_X_CHROMA_TE:
        case RZ_PID_ORNATA_CHROMA:
        case RZ_PID_ORNATA:
        case RZ_PID_BLADE_STEALTH_LATE_2016:
        case RZ_PID_BLACKWIDOW_CHROMA_V2:
        case RZ_PID_BLADE_LATE_2016:
        case RZ_PID_BLADE_PRO_2017:
        case RZ_PID_HUNTSMAN_ELITE:
        case RZ_PID_HUNTSMAN:
        case RZ_PID_BLACKWIDOW_ELITE:
        case RZ_PID_CYNOSA_CHROMA:
        case RZ_PID_TARTARUS_V2:
        case RZ_PID_CYNOSA_CHROMA_PRO:
        case RZ_PID_BLADE_STEALTH_MID_2017:
        case RZ_PID_BLADE_PRO_2017_FULLHD:
        case RZ_PID_BLADE_STEALTH_LATE_2017:
        case RZ_PID_BLADE_2018:
        case RZ_PID_BLADE_PRO_2019:
        case RZ_PID_BLACKWIDOW_LITE:
        case RZ_PID_BLACKWIDOW_ESSENTIAL:
        case RZ_PID_BLADE_STEALTH_2019:
        case RZ_PID_BLADE_2019_ADV:
        case RZ_PID_BLADE_2018_BASE:
        case RZ_PID_CYNOSA_LITE:
        case RZ_PID_BLADE_2018_MERCURY:
        case RZ_PID_BLACKWIDOW_2019:
        case RZ_PID_HUNTSMAN_TE:
        case RZ_PID_BLADE_MID_2019_MERCURY:
        case RZ_PID_BLADE_2019_BASE:
        case RZ_PID_BLADE_STEALTH_LATE_2019:
        case RZ_PID_BLADE_PRO_LATE_2019:
        case RZ_PID_BLADE_STUDIO_EDITION_2019:
        case RZ_PID_BLACKWIDOW_V3:
        case RZ_PID_BLADE_STEALTH_EARLY_2020:
        case RZ_PID_BLADE_15_ADV_2020:
        case RZ_PID_BLADE_EARLY_2020_BASE:
        case RZ_PID_BLADE_PRO_EARLY_2020:
        case RZ_PID_HUNTSMAN_MINI:
        case RZ_PID_BLACKWIDOW_V3_MINI:
        case RZ_PID_BLADE_STEALTH_LATE_2020:
        case RZ_PID_BLACKWIDOW_V3_PRO_WIRED:
        case RZ_PID_ORNATA_V2:
        case RZ_PID_CYNOSA_V2:
        case RZ_PID_HUNTSMAN_V2_ANALOG:
        case RZ_PID_HUNTSMAN_MINI_JP:
        case RZ_PID_BOOK_2020:
        case RZ_PID_HUNTSMAN_V2_TENKEYLESS:
        case RZ_PID_HUNTSMAN_V2:
        case RZ_PID_BLADE_15_ADV_EARLY_2021:
        case RZ_PID_BLADE_17_PRO_EARLY_2021:
        case RZ_PID_BLADE_15_BASE_EARLY_2021:
        case RZ_PID_BLADE_14_2021:
        case RZ_PID_BLACKWIDOW_V3_MINI_WIRELESS:
        case RZ_PID_BLADE_15_ADV_MID_2021:
        case RZ_PID_BLADE_17_PRO_MID_2021:
        case RZ_PID_BLADE_15_BASE_2022:
        case RZ_PID_HUNTSMAN_MINI_ANALOG:
        case RZ_PID_BLADE_15_ADV_EARLY_2022:
        case RZ_PID_BLADE_17_2022:
        case RZ_PID_BLADE_14_2022:
        case RZ_PID_BLACKWIDOW_V4_PRO:
        case RZ_PID_DEATHSTALKER_V2_PRO_WIRELESS:
        case RZ_PID_DEATHSTALKER_V2_PRO_WIRED:
        case RZ_PID_ORNATA_V3_X:
        case RZ_PID_DEATHSTALKER_V2:
        case RZ_PID_DEATHSTALKER_V2_PRO_TKL_WIRELESS:
        case RZ_PID_DEATHSTALKER_V2_PRO_TKL_WIRED:
        case RZ_PID_BLADE_15_2023:
        case RZ_PID_BLADE_16_2023:
        case RZ_PID_BLADE_18_2023:
        case RZ_PID_ORNATA_V3_X_ALT:
        case RZ_PID_BLACKWIDOW_V3_TK:
            return KEYBOARD;
        case RZ_PID_KRAKEN_CLASSIC:
        case RZ_PID_KRAKEN:
        case RZ_PID_KRAKEN_CLASSIC_ALT:
        case RZ_PID_KRAKEN_V2:
        case RZ_PID_KRAKEN_ULTIMATE:
            return KRAKEN;
        case RZ_PID_FIREFLY_HYPERFLUX:
        case RZ_PID_MOUSE_DOCK:
        case RZ_PID_CORE:
        case RZ_PID_NOMMO_CHROMA:
        case RZ_PID_NOMMO_PRO:
        case RZ_PID_FIREFLY:
        case RZ_PID_GOLIATHUS_CHROMA:
        case RZ_PID_GOLIATHUS_CHROMA_EXTENDED:
        case RZ_PID_FIREFLY_V2:
        case RZ_PID_CHROMA_MUG:
        case RZ_PID_CHROMA_BASE:
        case RZ_PID_CHROMA_HDK:
        case RZ_PID_LAPTOP_STAND_CHROMA:
        case RZ_PID_RAPTOR_27:
        case RZ_PID_KRAKEN_KITTY_EDITION:
        case RZ_PID_CORE_X_CHROMA:
        case RZ_PID_MOUSE_BUNGEE_V3_CHROMA:
        case RZ_PID_CHROMA_ADDRESSABLE_RGB_CONTROLLER:
        case RZ_PID_BASE_STATION_V2_CHROMA:
        case RZ_PID_THUNDERBOLT_4_DOCK_CHROMA:
        case RZ_PID_CHARGING_PAD_CHROMA:
        case RZ_PID_LAPTOP_STAND_CHROMA_V2:
            return ACCESSORY;
        default:
            fprintf(stderr, "Unknown Razer device with PID: %d", pid);
            return -1;
    }
}

static uint16_t rz_get_w_index(const uint16_t pid) {
    switch(pid) {
        case RZ_PID_BLACKWIDOW_CHROMA_V2:
            return 0x02;
        default:
            return 0x00;
    }
}

struct rz_device rz_open_device(const uint16_t pid) {
    struct rz_device dev;

    dev.pid = pid;
    dev.usb_dev = usb_get_device_by_pid(pid);
    dev.w_index = rz_get_w_index(pid);
    dev.type = rz_get_device_type(pid);

    // Claim the interface specified by wIndex
    if (libusb_claim_interface(dev.usb_dev, dev.w_index) < 0) {
        fprintf(stderr, "Failed to claim interface\n");
        libusb_close(dev.usb_dev);
        libusb_exit(NULL);
        return dev;
    }

    return dev;
}

void rz_close_device(struct rz_device dev) {
    libusb_release_interface(dev.usb_dev, dev.w_index);
    libusb_close(dev.usb_dev);
}