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
        let mut usb_device = USBDevice::new(RAZER_VENDOR_ID, product as u16);
        usb_device.open()?;

        //TODO: Claim interface here, see legacy code for reference. Also remember to release all interfaces.

        let rz_device = RazerDevice { usb_device };
        Ok(rz_device)
    }

    fn send_report(&self, report: RazerReport) -> Result<()> {
        let data: [u8; RZ_REPORT_LEN] = report.into();

        //TODO: Use more idiomatic constants
        self.usb_device
            .write_control(0x21, 0x09, 0x300, 0x00, &data, CONTROL_REPORT_TIMEOUT)?;

        Ok(())
    }
}
