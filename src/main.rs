use std::thread::sleep;

use chromacommon::{RzChromaDevice, RzRGB};

extern crate rusb;

mod rzcommon;
mod chromacommon;
mod rzdevices;

fn main() {
    let mut dev: RzChromaDevice = Default::default();
    dev.rz_device.open(0x0046);

    dev.set_brightness(1.0);

    sleep(std::time::Duration::from_millis(500));

    dev.set_effect_spectrum();
}