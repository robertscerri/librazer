use std::process::exit;

use rusb::{Context, DeviceHandle};

use crate::{rzdevices::*, usbcommon};

#[derive(Default, PartialEq, Eq)]
pub enum RzDeviceType {
    #[default]
    Mouse,
    Keyboard,
    Kraken,
    Accessory
}

#[derive(Default)]
pub struct RzDevice {
    pub usb_dev: Option<DeviceHandle<Context>>,
    pub pid: u16,
    pub w_index: u16,
    pub dev_type: RzDeviceType
}

#[derive(Default)]
pub struct RzReport {
    pub id: u8,
    pub cmd: u8,
    pub sub_cmd: u8,
    pub params: Vec<u8>
}

pub const RZ_VENDOR_ID: u16 = 0x1532;
const RZ_REPORT_LEN: usize = 90;

fn rz_calculate_crc(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;

    for i in 2..88 {
        crc ^= data[i];
    }

    return crc;
}

impl From<&RzReport> for [u8; RZ_REPORT_LEN] {
    fn from(report: &RzReport) -> Self {
        let mut data: [u8; RZ_REPORT_LEN] = [0; RZ_REPORT_LEN];

        let num_params: u8 = (report.params.len() as u8) + 1;
        let data_hdr: [u8; 7] = [0x00, report.id, 0x00, 0x00, 0x00, num_params, 0x03];

        let data_cmd: [u8; 2] = [report.cmd, report.sub_cmd];

        data[0..7].copy_from_slice(&data_hdr);
        data[7..9].copy_from_slice(&data_cmd);

        for (i, param) in report.params.iter().enumerate() {
            data[9 + i] = *param;
        }

        data[RZ_REPORT_LEN - 2] = rz_calculate_crc(&data);

        return data;
    }
}

impl RzDevice {
    fn get_w_index(&self) -> u16 {
        match self.pid {
            RZ_PID_BLACKWIDOW_CHROMA_V2 => 0x02,
            _ => 0x00
        }
    }

    fn get_device_type(&self) -> RzDeviceType {
        match self.pid {
            RZ_PID_IRICHU_2011 | 
            RZ_PID_DEATHADDER_3_5G | 
            RZ_PID_ABYSSUS_1800 | 
            RZ_PID_MAMBA_2012_WIRED | 
            RZ_PID_MAMBA_2012_WIRELESS | 
            RZ_PID_DEATHADDER_3_5G_BLACK | 
            RZ_PID_NAGA_2012 | 
            RZ_PID_IMPERATOR | 
            RZ_PID_OUROBOROS | 
            RZ_PID_TAIPAN | 
            RZ_PID_NAGA_HEX_RED | 
            RZ_PID_DEATHADDER_2013 | 
            RZ_PID_DEATHADDER_1800 | 
            RZ_PID_OROCHI_2013 | 
            RZ_PID_NAGA_EPIC_CHROMA | 
            RZ_PID_NAGA_EPIC_CHROMA_DOCK | 
            RZ_PID_NAGA_2014 | 
            RZ_PID_NAGA_HEX | 
            RZ_PID_ABYSSUS | 
            RZ_PID_DEATHADDER_CHROMA | 
            RZ_PID_MAMBA_WIRED | 
            RZ_PID_MAMBA_WIRELESS | 
            RZ_PID_MAMBA_TE_WIRED | 
            RZ_PID_OROCHI_CHROMA | 
            RZ_PID_DIAMONDBACK_CHROMA | 
            RZ_PID_DEATHADDER_2000 | 
            RZ_PID_NAGA_HEX_V2 | 
            RZ_PID_NAGA_CHROMA | 
            RZ_PID_DEATHADDER_3500 | 
            RZ_PID_LANCEHEAD_WIRED | 
            RZ_PID_LANCEHEAD_WIRELESS | 
            RZ_PID_ABYSSUS_V2 | 
            RZ_PID_DEATHADDER_ELITE | 
            RZ_PID_ABYSSUS_2000 | 
            RZ_PID_LANCEHEAD_TE_WIRED | 
            RZ_PID_ATHERIS_RECEIVER | 
            RZ_PID_BASILISK | 
            RZ_PID_BASILISK_ESSENTIAL | 
            RZ_PID_NAGA_TRINITY | 
            RZ_PID_ABYSSUS_ELITE_DVA_EDITION | 
            RZ_PID_ABYSSUS_ESSENTIAL | 
            RZ_PID_MAMBA_ELITE | 
            RZ_PID_DEATHADDER_ESSENTIAL | 
            RZ_PID_LANCEHEAD_WIRELESS_RECEIVER | 
            RZ_PID_LANCEHEAD_WIRELESS_WIRED | 
            RZ_PID_DEATHADDER_ESSENTIAL_WHITE_EDITION | 
            RZ_PID_MAMBA_WIRELESS_RECEIVER | 
            RZ_PID_MAMBA_WIRELESS_WIRED | 
            RZ_PID_PRO_CLICK_RECEIVER | 
            RZ_PID_VIPER | 
            RZ_PID_VIPER_ULTIMATE_WIRED | 
            RZ_PID_VIPER_ULTIMATE_WIRELESS | 
            RZ_PID_DEATHADDER_V2_PRO_WIRED | 
            RZ_PID_DEATHADDER_V2_PRO_WIRELESS | 
            RZ_PID_PRO_CLICK_WIRED | 
            RZ_PID_BASILISK_X_HYPERSPEED | 
            RZ_PID_DEATHADDER_V2 | 
            RZ_PID_BASILISK_V2 | 
            RZ_PID_BASILISK_ULTIMATE_WIRED | 
            RZ_PID_BASILISK_ULTIMATE_RECEIVER | 
            RZ_PID_VIPER_MINI | 
            RZ_PID_DEATHADDER_V2_MINI | 
            RZ_PID_NAGA_LEFT_HANDED_2020 | 
            RZ_PID_NAGA_PRO_WIRED | 
            RZ_PID_NAGA_PRO_WIRELESS | 
            RZ_PID_VIPER_8K | 
            RZ_PID_OROCHI_V2_RECEIVER | 
            RZ_PID_OROCHI_V2_BLUETOOTH | 
            RZ_PID_NAGA_X | 
            RZ_PID_DEATHADDER_ESSENTIAL_2021 | 
            RZ_PID_BASILISK_V3 | 
            RZ_PID_PRO_CLICK_MINI_RECEIVER | 
            RZ_PID_DEATHADDER_V2_X_HYPERSPEED | 
            RZ_PID_VIPER_MINI_SE_WIRED | 
            RZ_PID_VIPER_MINI_SE_WIRELESS | 
            RZ_PID_DEATHADDER_V2_LITE | 
            RZ_PID_COBRA | 
            RZ_PID_VIPER_V2_PRO_WIRED | 
            RZ_PID_VIPER_V2_PRO_WIRELESS | 
            RZ_PID_BASILISK_V3_PRO_WIRED | 
            RZ_PID_BASILISK_V3_PRO_WIRELESS | 
            RZ_PID_COBRA_PRO | 
            RZ_PID_DEATHADDER_V3 | 
            RZ_PID_HYPERPOLLING_WIRELESS_DONGLE | 
            RZ_PID_NAGA_V2_HYPERSPEED_RECEIVER | 
            RZ_PID_DEATHADDER_V3_PRO_WIRED | 
            RZ_PID_DEATHADDER_V3_PRO_WIRELESS | 
            RZ_PID_VIPER_V3_HYPERSPEED =>
                return RzDeviceType::Mouse,
            RZ_PID_BLACKWIDOW_ULTIMATE_2012 | 
            RZ_PID_BLACKWIDOW_STEALTH_EDITION | 
            RZ_PID_ANANSI | 
            RZ_PID_NOSTROMO | 
            RZ_PID_ORBWEAVER | 
            RZ_PID_DEATHSTALKER_ESSENTIAL | 
            RZ_PID_BLACKWIDOW_ULTIMATE_2013 | 
            RZ_PID_BLACKWIDOW_STEALTH | 
            RZ_PID_BLACKWIDOW_TE_2014 | 
            RZ_PID_TARTARUS | 
            RZ_PID_DEATHSTALKER_EXPERT | 
            RZ_PID_BLACKWIDOW_CHROMA | 
            RZ_PID_DEATHSTALKER_CHROMA | 
            RZ_PID_BLADE_STEALTH | 
            RZ_PID_ORBWEAVER_CHROMA | 
            RZ_PID_TARTARUS_CHROMA | 
            RZ_PID_BLACKWIDOW_CHROMA_TE | 
            RZ_PID_BLADE_QHD | 
            RZ_PID_BLADE_PRO_LATE_2016 | 
            RZ_PID_BLACKWIDOW_OVERWATCH | 
            RZ_PID_BLACKWIDOW_ULTIMATE_2016 | 
            RZ_PID_BLACKWIDOW_X_CHROMA | 
            RZ_PID_BLACKWIDOW_X_ULTIMATE | 
            RZ_PID_BLACKWIDOW_X_CHROMA_TE | 
            RZ_PID_ORNATA_CHROMA | 
            RZ_PID_ORNATA | 
            RZ_PID_BLADE_STEALTH_LATE_2016 | 
            RZ_PID_BLACKWIDOW_CHROMA_V2 | 
            RZ_PID_BLADE_LATE_2016 | 
            RZ_PID_BLADE_PRO_2017 | 
            RZ_PID_HUNTSMAN_ELITE | 
            RZ_PID_HUNTSMAN | 
            RZ_PID_BLACKWIDOW_ELITE | 
            RZ_PID_CYNOSA_CHROMA | 
            RZ_PID_TARTARUS_V2 | 
            RZ_PID_CYNOSA_CHROMA_PRO | 
            RZ_PID_BLADE_STEALTH_MID_2017 | 
            RZ_PID_BLADE_PRO_2017_FULLHD | 
            RZ_PID_BLADE_STEALTH_LATE_2017 | 
            RZ_PID_BLADE_2018 | 
            RZ_PID_BLADE_PRO_2019 | 
            RZ_PID_BLACKWIDOW_LITE | 
            RZ_PID_BLACKWIDOW_ESSENTIAL | 
            RZ_PID_BLADE_STEALTH_2019 | 
            RZ_PID_BLADE_2019_ADV | 
            RZ_PID_BLADE_2018_BASE | 
            RZ_PID_CYNOSA_LITE | 
            RZ_PID_BLADE_2018_MERCURY | 
            RZ_PID_BLACKWIDOW_2019 | 
            RZ_PID_HUNTSMAN_TE | 
            RZ_PID_BLADE_MID_2019_MERCURY | 
            RZ_PID_BLADE_2019_BASE | 
            RZ_PID_BLADE_STEALTH_LATE_2019 | 
            RZ_PID_BLADE_PRO_LATE_2019 | 
            RZ_PID_BLADE_STUDIO_EDITION_2019 | 
            RZ_PID_BLACKWIDOW_V3 | 
            RZ_PID_BLADE_STEALTH_EARLY_2020 | 
            RZ_PID_BLADE_15_ADV_2020 | 
            RZ_PID_BLADE_EARLY_2020_BASE | 
            RZ_PID_BLADE_PRO_EARLY_2020 | 
            RZ_PID_HUNTSMAN_MINI | 
            RZ_PID_BLACKWIDOW_V3_MINI | 
            RZ_PID_BLADE_STEALTH_LATE_2020 | 
            RZ_PID_BLACKWIDOW_V3_PRO_WIRED | 
            RZ_PID_ORNATA_V2 | 
            RZ_PID_CYNOSA_V2 | 
            RZ_PID_HUNTSMAN_V2_ANALOG | 
            RZ_PID_HUNTSMAN_MINI_JP | 
            RZ_PID_BOOK_2020 | 
            RZ_PID_HUNTSMAN_V2_TENKEYLESS | 
            RZ_PID_HUNTSMAN_V2 | 
            RZ_PID_BLADE_15_ADV_EARLY_2021 | 
            RZ_PID_BLADE_17_PRO_EARLY_2021 | 
            RZ_PID_BLADE_15_BASE_EARLY_2021 | 
            RZ_PID_BLADE_14_2021 | 
            RZ_PID_BLACKWIDOW_V3_MINI_WIRELESS | 
            RZ_PID_BLADE_15_ADV_MID_2021 | 
            RZ_PID_BLADE_17_PRO_MID_2021 | 
            RZ_PID_BLADE_15_BASE_2022 | 
            RZ_PID_HUNTSMAN_MINI_ANALOG | 
            RZ_PID_BLADE_15_ADV_EARLY_2022 | 
            RZ_PID_BLADE_17_2022 | 
            RZ_PID_BLADE_14_2022 | 
            RZ_PID_BLACKWIDOW_V4_PRO | 
            RZ_PID_DEATHSTALKER_V2_PRO_WIRELESS | 
            RZ_PID_DEATHSTALKER_V2_PRO_WIRED | 
            RZ_PID_ORNATA_V3_X | 
            RZ_PID_DEATHSTALKER_V2 | 
            RZ_PID_DEATHSTALKER_V2_PRO_TKL_WIRELESS | 
            RZ_PID_DEATHSTALKER_V2_PRO_TKL_WIRED | 
            RZ_PID_BLADE_15_2023 | 
            RZ_PID_BLADE_16_2023 | 
            RZ_PID_BLADE_18_2023 | 
            RZ_PID_ORNATA_V3_X_ALT | 
            RZ_PID_BLACKWIDOW_V3_TK =>
                return RzDeviceType::Keyboard,
            RZ_PID_KRAKEN_CLASSIC | 
            RZ_PID_KRAKEN | 
            RZ_PID_KRAKEN_CLASSIC_ALT | 
            RZ_PID_KRAKEN_V2 | 
            RZ_PID_KRAKEN_ULTIMATE =>
                return RzDeviceType::Kraken,
            RZ_PID_FIREFLY_HYPERFLUX | 
            RZ_PID_MOUSE_DOCK | 
            RZ_PID_CORE | 
            RZ_PID_NOMMO_CHROMA | 
            RZ_PID_NOMMO_PRO | 
            RZ_PID_FIREFLY | 
            RZ_PID_GOLIATHUS_CHROMA | 
            RZ_PID_GOLIATHUS_CHROMA_EXTENDED | 
            RZ_PID_FIREFLY_V2 | 
            RZ_PID_CHROMA_MUG | 
            RZ_PID_CHROMA_BASE | 
            RZ_PID_CHROMA_HDK | 
            RZ_PID_LAPTOP_STAND_CHROMA | 
            RZ_PID_RAPTOR_27 | 
            RZ_PID_KRAKEN_KITTY_EDITION | 
            RZ_PID_CORE_X_CHROMA | 
            RZ_PID_MOUSE_BUNGEE_V3_CHROMA | 
            RZ_PID_CHROMA_ADDRESSABLE_RGB_CONTROLLER | 
            RZ_PID_BASE_STATION_V2_CHROMA | 
            RZ_PID_THUNDERBOLT_4_DOCK_CHROMA | 
            RZ_PID_CHARGING_PAD_CHROMA | 
            RZ_PID_LAPTOP_STAND_CHROMA_V2 =>
                return RzDeviceType::Accessory,
            _ => {
                println!("Unknown Razer device with PID: {}", self.pid);
                exit(1);
            }
        }
    }

    pub fn open(&mut self, pid: u16) {
        self.pid = pid;
        self.usb_dev = usbcommon::usb_get_dev_by_pid(pid);
        self.w_index = self.get_w_index();
        self.dev_type = self.get_device_type();

        if self.usb_dev.is_none() {
            return;
        }

        //Trying to claim an interface on macOS gives an access error, however functions work as intended without claiming the interface.
        //To prevent access errors, the kernel driver must be detached, however this renders the device unusable.
        //This patch is placed here until a better solution is found.ß
        if (cfg!(target_os = "macos")) {
            return;
        }

        let res = self.usb_dev.as_ref().unwrap().claim_interface(self.w_index as u8);

        match res {
            Ok(_) => {},
            Err(e) => println!("Failed to claim interface: {:?}", e)
        }
    }

    pub fn close(&self) {
        if self.usb_dev.is_none() {
            return;
        }

        //Since for macOS targets no interfaces are claimed, none should thus be released.
        if (cfg!(target_os = "macos")) {
            return;
        }

        let res = self.usb_dev.as_ref().unwrap().release_interface(self.w_index as u8);

        match res {
            Ok(_) => {},
            Err(e) => println!("Failed to release interface: {:?}", e)
        }
    }

    pub fn send_report(&self, report: &RzReport) -> bool {
        if self.usb_dev.is_none() {
            return false;
        }

        let data: [u8; RZ_REPORT_LEN] = report.into();

        return self.usb_dev.as_ref().unwrap().write_control(
            0x21, 
            0x09, 
            0x300, 
            self.w_index, 
            &data,
            std::time::Duration::from_millis(2000)
        ).unwrap() == RZ_REPORT_LEN;
    }
}