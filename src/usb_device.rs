use rusb::{Context, Device, DeviceHandle, UsbContext};

use crate::utils::errors::{Error, Result};

#[derive(Debug)]
pub struct USBDevice {
    pub vendor_id: u16,
    pub product_id: u16,

    handle: Option<DeviceHandle<Context>>,
}

impl USBDevice {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        USBDevice {
            vendor_id,
            product_id,
            handle: None,
        }
    }

    pub fn open(&mut self) -> Result<()> {
        if self.handle.is_some() {
            return Err(Error::DeviceAlreadyOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            });
        }

        let device = find_device_by_vid_pid(self.vendor_id, self.product_id)?;

        if let Some(device) = device {
            let handle = device.open()?;
            self.handle = Some(handle);

            Ok(())
        } else {
            Err(Error::DeviceNotFound {
                vid: self.vendor_id,
                pid: self.product_id,
            })
        }
    }
}

fn find_device_by_vid_pid(vendor_id: u16, product_id: u16) -> Result<Option<Device<Context>>> {
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let device_descriptor = device.device_descriptor()?;

        if device_descriptor.vendor_id() == vendor_id
            && device_descriptor.product_id() == product_id
        {
            return Ok(Some(device));
        }
    }

    return Ok(None);
}
