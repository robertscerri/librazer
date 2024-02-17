#ifndef CHROMACOMMON_H
#define CHROMACOMMON_H

#include <stdint.h>
#include <stdbool.h>

#define RZ_CHROMA_EFFECT_NONE 0x00
#define RZ_CHROMA_EFFECT_WAVE 0x01
#define RZ_CHROMA_EFFECT_REACTIVE 0x02
//#define RZ_CHROMA_EFFECT_BREATH 0x03 //not implemented
#define RZ_CHROMA_EFFECT_SPECTRUM 0x04
//#define RZ_CHROMA_EFFECT_CUSTOM 0x05 // draw frame //not implemented
#define RZ_CHROMA_EFFECT_STATIC 0x06
//#define RZ_CHROMA_EFFECT_CLEAR_ROW 0x08 //not implemented

#define RZ_CHROMA_EFFECT_WAVE_DIRECTION_LEFT 0x01
#define RZ_CHROMA_EFFECT_WAVE_DIRECTION_RIGHT 0x02

struct rz_rgb {
    float r;
    float g;
    float b;
};

bool rz_set_brightness(const struct rz_device *dev, float brightness);
bool rz_set_effect(const struct rz_device *dev, const unsigned int effect_id, const uint8_t *params, const unsigned int params_len);
bool rz_set_effect_wave(const struct rz_device *dev, const uint8_t wave_direction);
bool rz_set_effect_reactive(const struct rz_device *dev, uint8_t speed, struct rz_rgb rgb);
bool rz_set_effect_spectrum(const struct rz_device *dev);
bool rz_set_effect_static(const struct rz_device *dev, struct rz_rgb rgb);

#endif //CHROMACOMMON_H