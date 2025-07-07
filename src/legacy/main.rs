use razer::{RzDevice, RzReport, RZ_PID_DEATHADDER_V3_PRO_WIRELESS};

pub fn main() {
    let mut device = RzDevice::default();
    device.open(RZ_PID_DEATHADDER_V3_PRO_WIRELESS);

    let report = RzReport {
        id: 0x1f,
        cmd_class: 0x07,
        cmd: 0x80,
        sub_cmd: 0x00,
        params: vec![0x00],
    };

    let success = device.send_report(&report);

    std::thread::sleep(std::time::Duration::from_micros(31000));

    let mut buf: [u8; 90] = [0; 90];

    let read_len = device
        .usb_dev
        .as_ref()
        .unwrap()
        .read_control(
            0xA1,
            0x01,
            0x300,
            device.w_index,
            &mut buf,
            std::time::Duration::from_millis(2000),
        )
        .unwrap();

    let report: RzReport = buf.into();

    let normalised = report.params[0] as f64 / u8::MAX as f64;
    let percentage = (normalised * 10000.0).round() / 100.0;
    println!("Deathadder V3 Pro Wireless Battery: {percentage}%");

    print!("\nPress any key to exit...");
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
}
