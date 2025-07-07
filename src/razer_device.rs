use crate::{
    razer_product::{RazerProduct, RAZER_VENDOR_ID},
    usb_device::USBDevice,
    utils::errors::Result,
};

pub struct RazerDevice {
    usb_device: USBDevice,
}

impl RazerDevice {
    pub fn new(product: RazerProduct) -> Result<Self> {
        let mut usb_device = USBDevice::new(RAZER_VENDOR_ID, product as u16);
        usb_device.open()?;

        let rz_device = RazerDevice { usb_device };
        Ok(rz_device)
    }
}
