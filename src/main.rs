use razer::{razer_device::RazerDevice, razer_product::RazerProduct};

pub fn main() {
    let mut device = RazerDevice::new(RazerProduct::DeathAdderV3ProWireless).unwrap();
    device.open().unwrap();
}
