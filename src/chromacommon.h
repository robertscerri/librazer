#ifndef CHROMACOMMON_H
#define CHROMACOMMON_H

#include <stdint.h>
#include <stdbool.h>

#define RZ_CHROMA_EFFECT_NONE 0x00
#define RZ_CHROMA_EFFECT_WAVE 0x01
#define RZ_CHROMA_EFFECT_REACTIVE 0x02
#define RZ_CHROMA_EFFECT_BREATH 0x03
#define RZ_CHROMA_EFFECT_SPECTRUM 0x04
#define RZ_CHROMA_EFFECT_CUSTOM 0x05 // draw frame
#define RZ_CHROMA_EFFECT_STATIC 0x06
#define RZ_CHROMA_EFFECT_CLEAR_ROW 0x08

bool rz_set_brightness(const struct rz_device *dev, float brightness);
bool rz_set_effect(const struct rz_device *dev, const unsigned int effect_id, const uint8_t *params, const unsigned int params_len);
bool rz_set_effect_static(const struct rz_device *dev, float r, float g, float b);

#endif //CHROMACOMMON_H
