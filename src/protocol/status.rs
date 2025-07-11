use crate::utils::errors::{Error, ProtcolError, Result};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Status {
    NewCommand,
    Busy,
    Success,
    Failure,
    NoResponseOrTimeout,
    NotSupported,
}

impl TryFrom<u8> for Status {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Status::NewCommand),
            0x01 => Ok(Status::Busy),
            0x02 => Ok(Status::Success),
            0x03 => Ok(Status::Failure),
            0x04 => Ok(Status::NoResponseOrTimeout),
            0x05 => Ok(Status::NotSupported),
            unknown => Err(ProtcolError::UnknownStatus(unknown).into()),
        }
    }
}
