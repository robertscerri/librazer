use razer::{
    capabilities::chroma_capable::ChromaCapable,
    razer_device::{Firefly, RazerDevice},
};

pub fn main() {
    let mut device = Firefly::new().unwrap();
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
