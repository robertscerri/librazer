use crate::{
    device::razer_device::{DeathAdderV3ProWireless, RazerDevice},
    protocol::razer_report::RazerReport,
    utils::errors::Result,
};

pub trait Rechargable: RazerDevice {
    fn get_battery_level(&mut self) -> Result<f32> {
        let params: Vec<u8> = vec![0x00, 0x00];

        //Try to get rid of this slice manipulation
        let mut argument_slice: [u8; 80] = [0; 80];
        argument_slice[0..params.len()].copy_from_slice(&params);

        //TODO: Use more idiomatic constants
        let report = RazerReport::new(
            0x00,
            0x1f,
            0x00,
            params.len() as u8,
            0x07,
            0x80,
            argument_slice,
        );

        self.send_report(report)?;

        //TODO: Investigate reason behind no battery percentage whilst charging
        //TODO: Find reasoning behind seemingly arbitrary sleep length
        std::thread::sleep(std::time::Duration::from_micros(3100));

        let report = self.read_report()?;
        println!("{report:?}");
        let normalised_percentage = report.arguments[1] as f32 / u8::MAX as f32;

        Ok(normalised_percentage)
    }

    //TODO: Get charging status
}

impl Rechargable for DeathAdderV3ProWireless {}
