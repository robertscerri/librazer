use rusb::{Context, DeviceHandle};

use crate::{rzdevices::*, usbcommon};

#[derive(Default)]
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

    pub fn open(&mut self, pid: u16) {
        self.pid = pid;
        self.usb_dev = usbcommon::usb_get_dev_by_pid(pid);
        self.w_index = self.get_w_index();
        self.dev_type = RzDeviceType::Keyboard; //TODO: Implement this

        if self.usb_dev.is_some() {
            let res = self.usb_dev.as_ref().unwrap().claim_interface(self.w_index as u8);

            match res {
                Ok(_) => {},
                Err(e) => println!("Failed to claim interface: {:?}", e)
            }
        }
    }

    pub fn close(&self) {
        if self.usb_dev.is_none() {
            return;
        }

        let res = self.usb_dev.as_ref().unwrap().release_interface(self.w_index as u8);

        match res {
            Ok(_) => {},
            Err(e) => println!("Failed to release interface: {:?}", e)
        }

        self.close();
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