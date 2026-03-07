#ifndef MIRROR_PLUGIN_H
#define MIRROR_PLUGIN_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
    IMAGE PLUGIN: MIRROR

    Действие: зеркальный разворот изображения.
    Параметры:
        - width: ширина изображения в пикселях
        - height: высота изображения в пикселях
        - rgba_data: буфер размера width * height * 4 (RGBA8)
        - params: параметры обработки в формате json:
            - horizontal: отразить по горизонтали
            - vertical: отразить по вертикали
*/
void process_image_impl(
    uint32_t width,
    uint32_t height,
    uint8_t *rgba_data,
    const char *params
);

#ifdef __cplusplus
}
#endif

#endif // MIRROR_H
