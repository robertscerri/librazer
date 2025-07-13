use std::{collections::HashSet, time::Duration};

use rusb::{Context, Device, DeviceHandle, UsbContext};

use crate::utils::errors::{Result, USBError};

#[derive(Debug)]
pub struct USBDevice {
    vendor_id: u16,
    product_id: u16,
    handle: Option<DeviceHandle<Context>>,
    claimed_interfaces: HashSet<u8>,
}

impl USBDevice {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        USBDevice {
            vendor_id,
            product_id,
            handle: None,
            claimed_interfaces: HashSet::new(),
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
            return Err(USBError::DeviceAlreadyOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into());
        }

        let device = find_device_by_vid_pid(self.vendor_id, self.product_id)?;

        match device {
            Some(device) => {
                let handle = device.open()?;
                self.handle = Some(handle);

                self.claimed_interfaces.clear();

                Ok(())
            }
            None => Err(USBError::DeviceNotFound {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into()),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.handle.is_some() {
            for iface in &self.claimed_interfaces {
                let _ = self.release_interface(*iface);
            }

            self.handle = None;

            Ok(())
        } else {
            Err(USBError::DeviceNotOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into())
        }
    }

    pub fn claim_interface(&self, iface: u8) -> Result<()> {
        if let Some(handle) = &self.handle {
            handle.claim_interface(iface)?;

            Ok(())
        } else {
            Err(USBError::DeviceNotOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into())
        }
    }

    pub fn release_interface(&self, iface: u8) -> Result<()> {
        if let Some(handle) = &self.handle {
            handle.release_interface(iface)?;

            Ok(())
        } else {
            Err(USBError::DeviceNotOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into())
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
            Err(USBError::DeviceNotOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into())
        }
    }

    pub fn read_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        length: usize,
        timeout: Duration,
    ) -> Result<Vec<u8>> {
        if let Some(handle) = &self.handle {
            let mut buf = vec![0; length];

            handle.read_control(request_type, request, value, index, &mut buf, timeout)?;

            Ok(buf)
        } else {
            Err(USBError::DeviceNotOpen {
                vid: self.vendor_id,
                pid: self.product_id,
            }
            .into())
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
