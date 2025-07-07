use razer::{razer_device::RazerDevice, razer_product::RazerProduct};

pub fn main() {
    let device = RazerDevice::new(RazerProduct::DeathAdderV3ProWireless).unwrap();
}
