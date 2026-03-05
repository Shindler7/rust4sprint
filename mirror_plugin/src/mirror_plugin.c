#include <stdint.h>
//#include "cJSON.h"

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
void process_image(
    int32_t width,
    int32_t height,
    uint8_t *rgba_data,
    const char* params
) {
    // TODO: реализация
}
