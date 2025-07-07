use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported operation, PID: {}", .pid)]
    UnsupportedOperation { pid: u16 },
    #[error("Device not found, VID: {}, PID: {}", .vid, .pid)]
    DeviceNotFound { vid: u16, pid: u16 },
    #[error("Device already open, VID: {}, PID: {}", .vid, .pid)]
    DeviceAlreadyOpen { vid: u16, pid: u16 },
    #[error("Device not open, VID: {}, PID: {}", .vid, .pid)]
    DeviceNotOpen { vid: u16, pid: u16 },
    #[error("USB error: {0}")]
    Usb(#[from] rusb::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
