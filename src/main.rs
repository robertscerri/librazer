use razer::{
    capabilities::{chroma_capable::ChromaCapable, rechargable::Rechargable},
    device::razer_device::{DeathAdderV3ProWireless, Firefly, RazerDevice},
};

pub fn main() {
    read_deathadder_battery();
}

fn dim_brightness_firefly() {
    let mut device = Firefly::new();
    device.open().unwrap();

    println!(
        "Connected to Razer Firefly (PID: {:#06X})",
        device.usb_device().product_id()
    );

    device.set_brightness(0.2).unwrap();
    std::thread::sleep(std::time::Duration::new(2, 0));
    device.set_brightness(1.0).unwrap();

    device.close().unwrap();
    println!("Closed connection with Razer Firefly");
}

fn read_deathadder_battery() {
    let mut mouse = DeathAdderV3ProWireless::new();
    mouse.open().unwrap();

    println!(
        "Connected to Razer DeathAdder V3 Pro Wireless (PID: {:#06X})",
        mouse.usb_device().product_id()
    );

    let battery = mouse.get_battery_level().unwrap();
    println!("Battery at: {:.2}%", battery * 100.0);

    mouse.close().unwrap();
    println!("Closed connection with Razer DeathAdder V3 Pro Wireless");
}
