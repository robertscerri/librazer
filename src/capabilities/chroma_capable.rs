use crate::{
    device::razer_device::{BlackWidowChromaV2, Firefly, RazerDevice},
    protocol::{razer_report::RazerReport, status::Status},
    utils::errors::Result,
};

pub trait ChromaCapable: RazerDevice {
    fn set_brightness(&mut self, brightness: f32) -> Result<()> {
        let brightness = brightness.clamp(0.0, 1.0);
        let params: Vec<u8> = vec![0x01, 0x05, (brightness * 255.0) as u8]; //TODO: Replace constants with idomatic values

        let report = RazerReport::new(Status::NewCommand, 0xff, 0x00, 0x03, 0x03, params);

        self.send_report(report)
    }
}

impl ChromaCapable for Firefly {}
impl ChromaCapable for BlackWidowChromaV2 {}
