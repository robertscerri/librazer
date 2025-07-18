use razer::{
    capabilities::{
        adjustable_dpi::AdjustableDPI, chroma_capable::ChromaCapable, rechargable::Rechargable,
    },
    device::razer_device::{BlackWidowChromaV2, DeathAdderV3ProWireless, Firefly, RazerDevice},
    protocol::constants::MatrixEffect,
};

pub fn main() {
    // dim_brightness_firefly();
    // dim_brightness_blackwidow();
    cycle_effects_blackwidow();
    // read_deathadder_battery();
    // read_deathadder_dpi();
}

fn dim_brightness_firefly() {
    let mut device = Firefly::new();
    device.open().unwrap();

    println!(
        "Connected to Razer Firefly (PID: {:#06X}, Interface: {:#04X})",
        device.usb_device().product_id(),
        device.interface_index()
    );

    device.set_brightness(0.2).unwrap();
    std::thread::sleep(std::time::Duration::new(2, 0));
    device.set_brightness(1.0).unwrap();

    device.close().unwrap();
    println!("Closed connection with Razer Firefly");
}

fn dim_brightness_blackwidow() {
    let mut device = BlackWidowChromaV2::new();
    device.open().unwrap();

    println!(
        "Connected to Razer BlackWidow Chroma V2 (PID: {:#06X}, Interface: {:#04X})",
        device.usb_device().product_id(),
        device.interface_index()
    );

    device.set_brightness(0.2).unwrap();
    std::thread::sleep(std::time::Duration::new(2, 0));
    device.set_brightness(1.0).unwrap();

    device.close().unwrap();
    println!("Closed connection with Razer BlackWidow Chroma V2");
}

fn cycle_effects_blackwidow() {
    let mut device = BlackWidowChromaV2::new();
    device.open().unwrap();

    println!(
        "Connected to Razer BlackWidow Chroma V2 (PID: {:#06X}, Interface: {:#04X})",
        device.usb_device().product_id(),
        device.interface_index()
    );

    device.set_effect(MatrixEffect::Off).unwrap();
    std::thread::sleep(std::time::Duration::new(2, 0));
    device.set_effect(MatrixEffect::Spectrum).unwrap();

    device.close().unwrap();
    println!("Closed connection with Razer BlackWidow Chroma V2");
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

fn read_deathadder_dpi() {
    let mut mouse = DeathAdderV3ProWireless::new();
    mouse.open().unwrap();

    println!(
        "Connected to Razer DeathAdder V3 Pro Wireless (PID: {:#06X})",
        mouse.usb_device().product_id()
    );

    let dpi = mouse.get_dpi().unwrap();
    println!("DPI (x, y): ({}, {})", dpi.0, dpi.1);

    mouse.close().unwrap();
    println!("Closed connection with Razer DeathAdder V3 Pro Wireless");
}
