use crate::{
    device::razer_device::{DeathAdderV3ProWireless, RazerDevice},
    protocol::{razer_report::RazerReport, status::Status},
    utils::errors::Result,
};

pub trait Rechargable: RazerDevice {
    //TODO: Investigate reason behind no battery percentage whilst charging
    fn get_battery_level(&mut self) -> Result<f32> {
        let params: Vec<u8> = vec![0x00, 0x00];

        //TODO: Device-specific transaction ID
        //TODO: Use more idiomatic constants
        let request = RazerReport::new(Status::NewCommand, 0x1F, 0x00, 0x07, 0x80, params);
        let response = self.exchange_report(request)?;

        let normalised_percentage = response.arguments[1] as f32 / u8::MAX as f32;

        Ok(normalised_percentage)
    }

    //TODO: Get charging status
}

impl Rechargable for DeathAdderV3ProWireless {}
