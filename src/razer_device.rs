use crate::{
    razer_product::{RazerProduct, RAZER_VENDOR_ID},
    razer_report::{RazerReport, RZ_REPORT_LEN},
    usb_device::USBDevice,
    utils::errors::Result,
};

const CONTROL_REPORT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(2000);

pub struct RazerDevice {
    usb_device: USBDevice,
}

impl RazerDevice {
    pub fn new(product: RazerProduct) -> Result<Self> {
        let usb_device = USBDevice::new(RAZER_VENDOR_ID, product as u16);

        let rz_device = RazerDevice { usb_device };
        Ok(rz_device)
    }

    pub fn open(&mut self) -> Result<()> {
        //TODO: Claim interface here, see legacy code for reference.
        self.usb_device.open()
    }

    pub fn close(&mut self) -> Result<()> {
        //TODO: Release interface here, see legacy code for reference.
        self.usb_device.close()
    }

    fn send_report(&self, report: RazerReport) -> Result<()> {
        let data: [u8; RZ_REPORT_LEN] = report.into();

        //TODO: Use more idiomatic constants
        self.usb_device
            .write_control(0x21, 0x09, 0x300, 0x00, &data, CONTROL_REPORT_TIMEOUT)?;

        Ok(())
    }
}
