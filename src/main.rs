use razer::{
    razer_product::{RazerProduct, RAZER_VENDOR_ID},
    usb_device::USBDevice,
};

pub fn main() {
    let mut device = USBDevice::new(
        RAZER_VENDOR_ID,
        RazerProduct::DeathAdderV3ProWireless as u16,
    );
    device.open().unwrap();
}
