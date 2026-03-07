//! Плагин MIRROR (написан на C).
//!
//! Обеспечивает вертикальное и (или) горизонтальное отражение изображения.
//!
//! Доступен метод: `process_image`.
//!
//! Ожидает параметры:
//! - `horizont` — отражение изображение по горизонту
//! - `vertical` — отражение изображение по вертикали

use std::ffi::c_char;

unsafe extern "C" {
    fn process_image_impl(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char);
}

/// # Safety
///
/// C ABI:
/// void process_image(uint32_t width, uint32_t height, uint8_t* rgba_data, const char* params)
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    process_image_impl(width, height, rgba_data, params);
}

#[cfg(test)]
mod tests {
    use super::process_image;
    use std::ffi::CString;
    use std::ptr;

    fn rgba(ids: &[u8]) -> Vec<u8> {
        ids.iter()
            // Каждый "пиксель" = 4 байта RGBA.
            .flat_map(|&id| [id, 0, 0, 255])
            .collect()
    }

    fn apply(width: u32, height: u32, ids: &[u8], params: Option<&str>) -> Vec<u8> {
        assert_eq!(ids.len(), (width * height) as usize);

        let mut data = rgba(ids);

        let c_params = params.map(|s| CString::new(s).expect("params must not contain NUL"));
        let params_ptr = c_params.as_ref().map_or(ptr::null(), |s| s.as_ptr());

        unsafe {
            process_image(width, height, data.as_mut_ptr(), params_ptr);
        }

        data
    }

    #[test]
    fn mirror_horizontal_2x2() {
        // 1 2
        // 3 4
        let out = apply(2, 2, &[1, 2, 3, 4], Some("horizont=1;vertical=0"));
        // 2 1
        // 4 3
        let expected = rgba(&[2, 1, 4, 3]);
        assert_eq!(out, expected);
    }

    #[test]
    fn mirror_vertical_2x2() {
        // 1 2
        // 3 4
        let out = apply(2, 2, &[1, 2, 3, 4], Some("horizont=0;vertical=1"));
        // 3 4
        // 1 2
        let expected = rgba(&[3, 4, 1, 2]);
        assert_eq!(out, expected);
    }

    #[test]
    fn mirror_both_2x2() {
        // 1 2
        // 3 4
        let out = apply(2, 2, &[1, 2, 3, 4], Some("horizont=1;vertical=1"));
        // 4 3
        // 2 1
        let expected = rgba(&[4, 3, 2, 1]);
        assert_eq!(out, expected);
    }

    #[test]
    fn mirror_horizontal_odd_width_center_stays() {
        // 1 2 3
        let out = apply(3, 1, &[1, 2, 3], Some("horizont=1;vertical=0"));
        // 3 2 1
        let expected = rgba(&[3, 2, 1]);
        assert_eq!(out, expected);
    }

    #[test]
    fn no_mirror_when_both_zero() {
        let out = apply(2, 2, &[1, 2, 3, 4], Some("horizont=0;vertical=0"));
        let expected = rgba(&[1, 2, 3, 4]);
        assert_eq!(out, expected);
    }

    #[test]
    fn null_rgba_is_safe_no_crash() {
        let params = CString::new("horizont=1;vertical=1").unwrap();
        unsafe {
            process_image(10, 10, ptr::null_mut(), params.as_ptr());
        }
    }
}
