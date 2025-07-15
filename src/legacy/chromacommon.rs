use std::thread::sleep;

use crate::rzcommon::{RzDevice, RzDeviceType, RzReport};

const RZ_CHROMA_EFFECT_CUSTOM: u8 = 0x05;
const RZ_CHROMA_EFFECT_CLEAR_ROW: u8 = 0x08;

#[derive(Default)]
pub struct RzChromaDevice {
    pub rz_device: RzDevice,
}

#[derive(Clone)]
pub struct RzRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RzRGB> for Vec<u8> {
    fn from(rgb: RzRGB) -> Self {
        vec![rgb.r, rgb.g, rgb.b]
    }
}

#[derive(Clone)]
pub struct RzMatrixRow {
    pub start: u8,
    pub rgb_values: Vec<RzRGB>,
}

impl RzChromaDevice {
    pub fn set_brightness(&self, mut brightness: f32) -> bool {
        brightness = brightness.clamp(0.0, 1.0);
        let params: Vec<u8> = vec![0x05, (brightness * 255.0) as u8];

        let report = RzReport {
            id: 0x1f,
            cmd_class: 0x03,
            cmd: 0x03,
            sub_cmd: 0x01,
            params,
        };

        return self.rz_device.send_report(&report);
    }

    pub fn set_effect(&self, effect_id: u8, params: Vec<u8>) -> bool {
        let report = RzReport {
            id: 0x1f,
            cmd_class: 0x03,
            cmd: 0x0a,
            sub_cmd: effect_id,
            params,
        };

        return self.rz_device.send_report(&report);
    }

    pub fn set_effect_custom(&self, matrix: Vec<RzMatrixRow>) -> bool {
        for i in 0..matrix.len() {
            let row = matrix.get(i).unwrap();
            let row_len: usize = row.rgb_values.len();

            let mut report = RzReport::default();
            report.id = 0x1f;

            let mut params: Vec<u8> = Vec::new();

            //Add device type specific parameters and commands
            if self.rz_device.dev_type == RzDeviceType::Keyboard {
                params.append(&mut vec![
                    i as u8,
                    row.start,
                    (row_len as u8 + row.start - 1),
                ]);

                report.cmd = 0x0b;
                report.sub_cmd = 0xff;
            } else {
                params.append(&mut vec![row_len as u8 + row.start - 1]);

                report.cmd = 0x0c;
                report.sub_cmd = row.start;
            }

            //Append RGB values to parameters
            for j in 0..row_len {
                let rgb = row.rgb_values.get(j).unwrap();
                params.append(&mut vec![rgb.r, rgb.g, rgb.b]);
            }

            //Send report
            report.params = params;
            self.rz_device.send_report(&report);

            //Sleep to give device time to get ready for next request
            sleep(std::time::Duration::from_micros(5000));
        }

        return self.set_effect(RZ_CHROMA_EFFECT_CUSTOM, Vec::new());
    }

    pub fn set_effect_clear_row(&self, row: u8) -> bool {
        return self.set_effect(RZ_CHROMA_EFFECT_CLEAR_ROW, vec![row]);
    }
}
