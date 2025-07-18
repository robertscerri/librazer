use crate::{
    device::razer_device::{DeathAdderV3ProWireless, RazerDevice},
    protocol::{constants::LedStorage, razer_report::RazerReport, status::Status},
    utils::errors::Result,
};

pub trait AdjustableDPI: RazerDevice {
    fn get_dpi(&mut self) -> Result<(u16, u16)> {
        let mut params = vec![LedStorage::NoStore as u8];
        params.resize(7, 0x00); //Required since device seems to respond with same length

        //TODO: Device-specific transaction ID
        //TODO: Use more idiomatic constants
        let request = RazerReport::new(Status::NewCommand, 0x1F, 0x00, 0x04, 0x85, params);
        let response = self.exchange_report(request)?;

        //TODO: Not all mice return DPI in this format
        let dpi_x = u16::from_be_bytes([response.arguments[1], response.arguments[2]]);
        let dpi_y = u16::from_be_bytes([response.arguments[3], response.arguments[4]]);

        Ok((dpi_x, dpi_y))
    }

    fn set_dpi(&mut self, dpi: (u16, u16)) -> Result<()> {
        todo!("attempted to set DPI to ({}, {})", dpi.0, dpi.1);
    }

    //TODO: DPI Stages?
    // fn get_dpi_stages(&mut self) -> Result<()> {
    //     let mut params = vec![LedStorage::VarStore as u8];
    //     params.resize(0x26, 0x00);

    //     let request = RazerReport::new(Status::NewCommand, 0x1F, 0x00, 0x04, 0x86, params);

    //     let response = self.exchange_report(request)?;

    //     println!("{response:?}");

    //     todo!()
    // }
}

impl AdjustableDPI for DeathAdderV3ProWireless {}
