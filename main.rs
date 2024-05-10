use crate::rzcommon::RzDevice;

extern crate rusb;

mod rzcommon;
mod chromacommon;
mod rzdevices;

fn main() {
    

    println!("{:02x?}", chromacommon::rz_set_brightness(&dev, 0.5));
}
