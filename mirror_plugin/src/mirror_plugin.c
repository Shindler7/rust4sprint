#include "mirror_plugin.h"

#include <stdbool.h>
#include <stddef.h>
#include <string.h>

// Обработка одного элемента конфигурации (простой сценарий).
static bool parse_param_bool(const char *params, const char *key) {
    if (!params || !key) return false;

    size_t key_len = strlen(key);
    const char *ptr = strstr(params, key);
    if (ptr) {
        ptr += key_len;
        return (*ptr == '1');
    }
    return false;
}

// Парсинг переданных параметров.
static void parse_params(const char *params, bool *horizontal, bool *vertical) {
    const char HORIZONT_KEY[] = "horizont=";
    const char VERTICAL_KEY[] = "vertical=";

    *horizontal = parse_param_bool(params, HORIZONT_KEY);
    *vertical = parse_param_bool(params, VERTICAL_KEY);
}

// Меняет местами два пикселя RGBA (по 4 байта каждый) по адресам `a` и `b`.
static inline void swap_rgba(uint8_t *a, uint8_t *b) {
    uint8_t t0 = a[0], t1 = a[1], t2 = a[2], t3 = a[3];
    a[0] = b[0]; a[1] = b[1]; a[2] = b[2]; a[3] = b[3];
    b[0] = t0;   b[1] = t1;   b[2] = t2;   b[3] = t3;
}

/*
    IMAGE PLUGIN: MIRROR
*/
void process_image_impl(
    uint32_t width,
    uint32_t height,
    uint8_t *rgba_data,
    const char *params
) {

    if (!rgba_data) return;

    // Параметры обработки.
    bool horizontal = true;
    bool vertical = false;
    parse_params(params, &horizontal, &vertical);

    // Основная обработка.

    // Горизонтальный.
    if (horizontal) {
        for (uint32_t y = 0; y < height; y++) {
            for (uint32_t x = 0; x < width / 2; x++) {
                size_t i1 = ((size_t)y * width + x) * 4;
                size_t i2 = ((size_t)y * width + (width - 1u - x)) * 4;
                swap_rgba(&rgba_data[i1], &rgba_data[i2]);
            }
        }
    }

    // Вертикальный.
    if (vertical) {
        for (uint32_t y = 0; y < height / 2; y++) {
            for (uint32_t x = 0; x < width; x++) {
                size_t i1 = ((size_t)y * width + x) * 4;
                size_t i2 = ((size_t)(height - 1u - y) * width + x) * 4;
                swap_rgba(&rgba_data[i1], &rgba_data[i2]);
            }
        }
    }
}
