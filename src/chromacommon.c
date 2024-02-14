#include "chromacommon.h"

#include "rzcommon.h"

static void clampf(float *val, float min, float max) {
    if (*val > max) {
        *val = max;
    } else if (*val < min) {
        *val = min;
    }
}

static void clamp_u8(uint8_t *val, uint8_t min, uint8_t max) {
    if (*val > max) {
        *val = max;
    } else if (*val < min) {
        *val = min;
    }
}

bool rz_set_brightness(const struct rz_device *dev, float brightness) {
    clampf(&brightness, 0, 1);
    uint8_t params[2] = {0x05, (brightness * 255)};

    struct rz_report report;
    report.id = 0x1f;
    report.cmd = 0x03;
    report.sub_cmd = 0x01;
    report.params = params;
    report.params_len = 2;

    return rz_send_report(dev, &report) > 0;
}

bool rz_set_effect(const struct rz_device *dev, const unsigned int effect_id, const uint8_t *params, const unsigned int params_len) {
    struct rz_report report;
    report.id = 0x1f;
    report.cmd = 0x0a;
    report.sub_cmd = effect_id;
    report.params = (unsigned char *) params;
    report.params_len = params_len;

    return rz_send_report(dev, &report) > 0;
}

bool rz_set_effect_wave(const struct rz_device *dev, const uint8_t wave_direction) {
    if (wave_direction != RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT && wave_direction != RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT) {
        return false;
    }

    uint8_t params[1] = {wave_direction};

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_WAVE, params, 1);
}

bool rz_set_effect_reactive(const struct rz_device *dev, uint8_t speed, struct rz_rgb rgb) {
    clamp_u8(&speed, 0x01, 0x03);

    clampf(&rgb.r, 0, 1);
    clampf(&rgb.g, 0, 1);
    clampf(&rgb.b, 0, 1);

    const uint8_t r8 = rgb.r * 255;
    const uint8_t g8 = rgb.g * 255;
    const uint8_t b8 = rgb.b * 255;

    const uint8_t params[4] = {speed, r8, g8, b8};

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_REACTIVE, params, 4);
}

bool rz_set_effect_spectrum(const struct rz_device *dev) {
    return rz_set_effect(dev, RZ_CHROMA_EFFECT_SPECTRUM, NULL, 0);
}

bool rz_set_effect_static(const struct rz_device *dev, struct rz_rgb rgb) {
    clampf(&rgb.r, 0, 1);
    clampf(&rgb.g, 0, 1);
    clampf(&rgb.b, 0, 1);

    const uint8_t r8 = rgb.r * 255;
    const uint8_t g8 = rgb.g * 255;
    const uint8_t b8 = rgb.b * 255;

    const uint8_t params[3] = {r8, g8, b8};

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_STATIC, params, 3);
}