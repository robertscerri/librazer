use crate::{chromacommon::rz_set_brightness, rzcommon::RzDevice};

extern crate rusb;

mod rzcommon;
mod chromacommon;
mod rzdevices;

fn main() {
    let mut dev = RzDevice {
        pid: 0,
        usb_dev: None,
        w_index: 0,
        dev_type: rzcommon::RzDeviceType::Mouse
    };
    dev.open(0x0046);

    rz_set_brightness(&dev, 1.0);
}
