use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("USB error: {0}")]
    Usb(#[from] USBError),
    #[error("Prtocol error: {0}")]
    Protocol(#[from] ProtcolError),
}

#[derive(Debug, Error)]
pub enum USBError {
    #[error("Backend error: {0}")]
    Backend(rusb::Error),
    #[error("Device not found, VID: {}, PID: {}", .vid, .pid)]
    DeviceNotFound { vid: u16, pid: u16 },
    #[error("Device already open, VID: {}, PID: {}", .vid, .pid)]
    DeviceAlreadyOpen { vid: u16, pid: u16 },
    #[error("Device not open, VID: {}, PID: {}", .vid, .pid)]
    DeviceNotOpen { vid: u16, pid: u16 },
}

#[derive(Debug, Error)]
pub enum ProtcolError {
    #[error("Unknown status: {0}")]
    UnknownStatus(u8),
}

impl From<rusb::Error> for Error {
    fn from(error: rusb::Error) -> Self {
        USBError::Backend(error).into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
