#include "rzcommon.h"
#include "chromacommon.h"

static void clampf(float *val, uint8_t min, uint8_t max) {
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
    report.params = params;
    report.params_len = params_len;

    return rz_send_report(dev, &report) > 0;
}

bool rz_set_effect_static(const struct rz_device *dev, float r, float g, float b) {
    clampf(&r, 0, 1);
    clampf(&g, 0, 1);
    clampf(&b, 0, 1);

    uint8_t r8 = r * 255;
    uint8_t g8 = g * 255;
    uint8_t b8 = b * 255;

    uint8_t params[3] = {r8, g8, b8};

    return rz_set_effect(dev, RZ_CHROMA_EFFECT_STATIC, params, 3);
}