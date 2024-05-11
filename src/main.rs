extern crate rusb;

use std::thread::sleep;

use chromacommon::{RzChromaDevice, RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT, RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT};
use rzdevices::RZ_PID_BLACKWIDOW_CHROMA_V2;

mod rzcommon;
mod chromacommon;
mod rzdevices;
mod usbcommon;

fn main() {
    let mut dev: RzChromaDevice = Default::default();
    dev.rz_device.open(RZ_PID_BLACKWIDOW_CHROMA_V2);

    dev.set_brightness(1.0);
    sleep(std::time::Duration::from_millis(500));
    dev.set_effect_wave(RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT);

    dev.rz_device.close();
}