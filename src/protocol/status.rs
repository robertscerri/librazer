use crate::utils::errors::{Error, ProtcolError, Result};

#[derive(Debug)]
pub enum Status {
    NewCommand,
    Busy,
    Success,
    Failure,
    NoResponseOrTimeout,
    NotSupported,
}

impl Status {
    pub fn as_u8(&self) -> u8 {
        match self {
            Status::NewCommand => 0x00,
            Status::Busy => 0x01,
            Status::Success => 0x02,
            Status::Failure => 0x03,
            Status::NoResponseOrTimeout => 0x04,
            Status::NotSupported => 0x05,
        }
    }
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
