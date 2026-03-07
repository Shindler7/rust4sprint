//! Плагин BLUR (написан на Rust).
//!
//! Обеспечивает размытие изображения.
//!
//! Доступен метод: `process_image`.
//!
//! Ожидает параметры:
//! - `radius` — радиус размытия (по умолчанию = 1)
//! - `iterations` — количество итераций (по умолчанию = 10).

mod params;
mod process;

use process::{Rgba, blur_executor};
use std::{ffi::c_char, slice::from_raw_parts_mut};

/// C ABI:
/// void process_image(uint32_t width, uint32_t height, uint8_t* rgba_data, const char* params).
#[unsafe(no_mangle)]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    if rgba_data.is_null() || width == 0 || height == 0 {
        return;
    }

    // Формально дублирование перепроверки, но это право плагина.
    let width = width as usize;
    let height = height as usize;
    let Some(size) = width.checked_mul(height) else {
        return;
    };

    let param = params::Params::from_params(params);
    let pixels = unsafe { from_raw_parts_mut(rgba_data.cast::<Rgba>(), size) };
    blur_executor(pixels, width, height, &param);
}

#[cfg(test)]
mod tests {
    use super::process_image;
    use std::ffi::CString;
    use std::ptr;

    fn px(v: u8) -> [u8; 4] {
        [v, v, v, v]
    }

    fn img(values: &[u8]) -> Vec<u8> {
        values.iter().flat_map(|&v| px(v)).collect()
    }

    fn run(width: u32, height: u32, values: &[u8], params: Option<&str>) -> Vec<u8> {
        assert_eq!(values.len(), (width * height) as usize);

        let mut data = img(values);
        let c_params = params.map(|s| CString::new(s).expect("params must not contain NUL"));
        let params_ptr = c_params.as_ref().map_or(ptr::null(), |s| s.as_ptr());

        process_image(width, height, data.as_mut_ptr(), params_ptr);
        data
    }

    #[test]
    fn blur_1x1_unchanged() {
        let out = run(1, 1, &[42], Some("radius=5,iterations=10"));
        assert_eq!(out, img(&[42]));
    }

    #[test]
    fn blur_2x1_radius1_iter1_expected_values() {
        // Для [0, 255], radius=1, iter=1:
        // left  = (2*0 + 255) / 3 = 85
        // right = (2*255 + 0) / 3 = 170
        let out = run(2, 1, &[0, 255], Some("radius=1,iterations=1"));
        assert_eq!(out, img(&[85, 170]));
    }

    #[test]
    fn iterations_zero_no_change() {
        let out = run(3, 1, &[10, 20, 30], Some("radius=2,iterations=0"));
        assert_eq!(out, img(&[10, 20, 30]));
    }

    #[test]
    fn radius_zero_no_change() {
        let out = run(3, 1, &[10, 20, 30], Some("radius=0,iterations=5"));
        assert_eq!(out, img(&[10, 20, 30]));
    }

    #[test]
    fn null_params_uses_defaults() {
        // По текущему коду default: radius=1, iterations=100.
        // Для 2 пикселей [0,255] после многих итераций стабилизируется в [127,128].
        let out = run(2, 1, &[0, 255], None);
        assert_eq!(out, img(&[127, 128]));
    }

    #[test]
    fn null_rgba_is_safe() {
        let params = CString::new("radius=1,iterations=1").unwrap();
        process_image(10, 10, ptr::null_mut(), params.as_ptr());
    }

    #[test]
    fn zero_dimensions_are_safe() {
        let mut data = img(&[1, 2, 3, 4]);
        let params = CString::new("radius=1,iterations=1").unwrap();

        process_image(0, 2, data.as_mut_ptr(), params.as_ptr());
        process_image(2, 0, data.as_mut_ptr(), params.as_ptr());

        // Должно остаться без изменений.
        assert_eq!(data, img(&[1, 2, 3, 4]));
    }
}
