use rusb::{Context, DeviceHandle, UsbContext};

use crate::rzcommon::RZ_VENDOR_ID;

pub fn usb_get_dev_by_pid(pid: u16) -> Option<DeviceHandle<Context>> {
    let context = Context::new().unwrap();
    
    for device in context.devices().unwrap().iter() {
        match device.device_descriptor() {
            Ok(desc) => {
                if desc.vendor_id() == RZ_VENDOR_ID && desc.product_id() == pid {
                    match device.open() {
                        Ok(handle) => return Some(handle),
                        Err(e) => println!("Failed to open device: {:?}", e)
                    }
                }
            }
            Err(_) => continue
        }
    }
    None
}