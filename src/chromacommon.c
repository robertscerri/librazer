#include "rzcommon.h"
#include <unistd.h>

#include "chromacommon.h"

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
    report.params = (uint8_t *) params;
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

    const uint8_t params[4] = {speed, rgb.r, rgb.g, rgb.b};

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_REACTIVE, params, 4);
}

bool rz_set_effect_spectrum(const struct rz_device *dev) {
    return rz_set_effect(dev, RZ_CHROMA_EFFECT_SPECTRUM, NULL, 0);
}

bool rz_set_effect_static(const struct rz_device *dev, struct rz_rgb rgb) {
    const uint8_t params[3] = {rgb.r, rgb.g, rgb.b};

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_STATIC, params, 3);
}

bool rz_set_colour(const struct rz_device *dev, struct rz_rgb rgb) {
    const int params_len = 46;
    uint8_t params[params_len];
    params[0] = 0x0e;

    for (int i = 1; i < params_len; i+=3) {
        params[i] = rgb.r;
        params[i+1] = rgb.g;
        params[i+2] = rgb.b;
    }

    struct rz_report report;
    report.id = 0x1f;
    report.cmd = 0x0c;
    report.sub_cmd = 0x00;
    report.params = (uint8_t *) params;
    report.params_len = params_len;

    rz_send_report(dev, &report);
    usleep(50000);
    rz_set_effect(dev, RZ_CHROMA_EFFECT_CUSTOM, NULL, 0);

    return 0;
}

bool rz_set_colour_kbd(const struct rz_device *dev, struct rz_rgb rgb) {
    for (int i = 0; i <= 5; i++) {
        const int params_len = 67;
        uint8_t params[params_len];
        params[0] = i;
        params[1] = 0x00;
        params[2] = 0x15;

        for (int i = 3; i < params_len; i+=3) {
            params[i] = rgb.r;
            params[i+1] = rgb.g;
            params[i+2] = rgb.b;
        }

        struct rz_report report;
        report.id = 0x1f;
        report.cmd = 0x0b;
        report.sub_cmd = 0xff;
        report.params = (uint8_t *) params;
        report.params_len = params_len;

        rz_send_report(dev, &report);
        usleep(5000);
    }
    rz_set_effect(dev, RZ_CHROMA_EFFECT_CUSTOM, NULL, 0);

    return 0;
}