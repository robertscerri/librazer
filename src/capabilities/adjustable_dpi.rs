use crate::{
    device::razer_device::{DeathAdderV3ProWireless, RazerDevice},
    protocol::{constants::LedStorage, razer_report::RazerReport, status::Status},
    utils::errors::Result,
};

pub trait AdjustableDPI: RazerDevice {
    fn get_dpi(&mut self) -> Result<(u16, u16)> {
        //TODO: Not really sure what's going on here
        let params = vec![
            LedStorage::NoStore as u8,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        //TODO: Device-specific transaction ID
        //TODO: Use more idiomatic constants
        let report = RazerReport::new(Status::NewCommand, 0x1f, 0x00, 0x04, 0x85, params);

        self.send_report(report)?;

        //TODO: Investigate reason behind no battery percentage whilst charging
        //TODO: Find reasoning behind seemingly arbitrary sleep length
        std::thread::sleep(std::time::Duration::from_micros(3100));

        let report = self.read_report()?;

        //TODO: Not all mice return DPI in this format
        let dpi_x = u16::from_be_bytes([report.arguments[1], report.arguments[2]]);
        let dpi_y = u16::from_be_bytes([report.arguments[3], report.arguments[4]]);

        Ok((dpi_x, dpi_y))
    }

    fn set_dpi(&mut self, dpi: (u16, u16)) -> Result<()> {
        todo!("attempted to set DPI to ({}, {})", dpi.0, dpi.1);
    }

    //TODO: DPI Stages?
}

impl AdjustableDPI for DeathAdderV3ProWireless {}
