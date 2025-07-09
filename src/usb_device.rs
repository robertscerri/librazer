use std::time::Duration;

use rusb::{Context, Device, DeviceHandle, UsbContext};

use crate::utils::errors::{Error, Result};

#[derive(Debug)]
pub struct USBDevice {
    vendor_id: u16,
    product_id: u16,
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

    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub fn product_id(&self) -> u16 {
        self.product_id
    }

    pub fn open(&mut self) -> Result<()> {
        if self.handle.is_some() {
            return Err(Error::DeviceAlreadyOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            });
        }

        let device = find_device_by_vid_pid(self.vendor_id, self.product_id)?;

        match device {
            Some(device) => {
                let handle = device.open()?;
                self.handle = Some(handle);

                Ok(())
            }
            None => Err(Error::DeviceNotFound {
                vid: self.vendor_id,
                pid: self.product_id,
            }),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        match &self.handle {
            Some(handle) => {
                //TODO: Write device close code here (release interfaces?)
                self.handle = None;

                Ok(())
            }
            None => Err(Error::DeviceNotOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }),
        }
    }

    pub fn write_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
        timeout: Duration,
    ) -> Result<usize> {
        if let Some(handle) = &self.handle {
            Ok(handle.write_control(request_type, request, value, index, buf, timeout)?)
        } else {
            Err(Error::DeviceNotOpen {
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
