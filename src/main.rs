extern crate rusb;

use std::{thread::sleep, time::Duration};

use chromacommon::{RzChromaDevice, RzMatrixRow, RzRGB, RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT, RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT};
use rzdevices::RZ_PID_BLACKWIDOW_CHROMA_V2;

mod rzcommon;
mod chromacommon;
mod rzdevices;
mod usbcommon;

fn main() {
    let mut dev: RzChromaDevice = RzChromaDevice::default();
    dev.rz_device.open(RZ_PID_BLACKWIDOW_CHROMA_V2);

    demo_run(&dev);

    // test_matrix(&dev);

    dev.rz_device.close();
}

fn demo_run(dev: &RzChromaDevice) {
    dev.set_brightness(0.2);
    sleep(Duration::from_millis(500));
    dev.set_brightness(0.8);
    sleep(Duration::from_millis(500));
    dev.set_brightness(0.0);
    sleep(Duration::from_millis(500));
    dev.set_brightness(1.0);

    dev.set_effect_wave(RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT);
    sleep(Duration::from_secs(2));
    dev.set_effect_wave(RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT);
    sleep(Duration::from_secs(2));
    dev.set_effect_breath_random();
    sleep(Duration::from_secs(5));
    dev.set_effect_static(RzRGB { r: 0xff, g: 0, b: 0 });
    sleep(Duration::from_secs(4));
    dev.set_effect_static(RzRGB { r: 0, g: 0xff, b: 0 });
    sleep(Duration::from_secs(4));
    dev.set_effect_static(RzRGB { r: 0, g: 0, b: 0xff });
    sleep(Duration::from_secs(4));
    dev.set_effect_static(RzRGB { r: 0xff, g: 0xff, b: 0 });
    sleep(Duration::from_secs(4));
    dev.set_effect_static(RzRGB { r: 0xff, g: 0, b: 0xff });
    sleep(Duration::from_secs(4));
    dev.set_effect_static(RzRGB { r: 0, g: 0xff, b: 0xff });
    sleep(Duration::from_secs(4));
    dev.set_effect_spectrum();
}

fn test_matrix(dev: &RzChromaDevice) {
    let blank_row = RzMatrixRow {
        start: 0,
        rgb_values: vec![
            RzRGB{r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB{r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00}
        ]
    };

    let test_row = RzMatrixRow {
        start: 0,
        rgb_values: vec![
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB{r: 0xff, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0xff, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0xff},
            RzRGB {r: 0xff, g: 0x00, b: 0xff},
            RzRGB {r: 0xff, g: 0xff, b: 0x00},
            RzRGB {r: 0x00, g: 0xff, b: 0xff},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00},
            RzRGB {r: 0x00, g: 0x00, b: 0x00}
        ]
    };

    dev.set_effect_custom(vec![
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        test_row.clone(),
        blank_row.clone(),
        blank_row.clone()
    ]);
}