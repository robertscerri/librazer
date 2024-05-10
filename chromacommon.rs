use crate::rzcommon::{RzDevice, RzReport};

const RZ_CHROMA_EFFECT_NONE: u8 = 0x00;
const RZ_CHROMA_EFFECT_WAVE: u8 = 0x01;
const RZ_CHROMA_EFFECT_REACTIVE: u8 = 0x02;
const RZ_CHROMA_EFFECT_BREATH: u8 = 0x03;
const RZ_CHROMA_EFFECT_SPECTRUM: u8 = 0x04;
const RZ_CHROMA_EFFECT_CUSTOM: u8 = 0x05;
const RZ_CHROMA_EFFECT_STATIC: u8 = 0x06;

const RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT: u8 = 0x01;
const RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT: u8 = 0x02;

pub fn rz_set_brightness(dev: &RzDevice, brightness: f32) -> bool {
    brightness.clamp(0.0, 1.0);
    let params: Vec<u8> = vec![0x05, (brightness * 255.0) as u8];

    let report = RzReport {
        id: 0x1f,
        cmd: 0x03,
        sub_cmd: 0x01,
        params
    };

    return dev.send_report(&report);
}

pub fn rz_set_effect(dev: &RzDevice, effect_id: u8, params: Vec<u8>) -> bool {
    let report = RzReport {
        id: 0x1f,
        cmd: 0x0a,
        sub_cmd: effect_id,
        params
    };

    return dev.send_report(&report);
}

pub fn rz_set_effect_wave(dev: &RzDevice, wave_direction: u8) -> bool {
    if wave_direction != RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT && wave_direction != RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT {
        return false;
    }

    let params: Vec<u8> = vec![wave_direction];

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_WAVE, params);
}

pub fn rz_set_effect_breath_random(dev: &RzDevice) -> bool {
    let params: Vec<u8> = vec![0x03];

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_BREATH, params);
}

pub fn rz_set_effect_spectrum(dev: &RzDevice) -> bool {
    return rz_set_effect(dev, RZ_CHROMA_EFFECT_SPECTRUM, Vec::new());
}