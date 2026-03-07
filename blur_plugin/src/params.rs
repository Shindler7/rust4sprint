//! Обработка и формирование параметров работы плагина.

use std::ffi::{CStr, c_char};

/// Значение по умолчанию для радиуса размытия.
const DEFAULT_RADIUS: isize = 1;
/// Значение по умолчанию для количества итераций.
const DEFAULT_ITERATIONS: usize = 100;

/// Параметры размытия изображения.
pub(super) struct Params {
    /// Радиус размытия.
    pub(super) radius: isize,
    /// Количество итераций.
    pub(super) iterations: usize,
}

impl Default for Params {
    fn default() -> Params {
        Self {
            radius: DEFAULT_RADIUS,
            iterations: DEFAULT_ITERATIONS,
        }
    }
}

impl Params {
    /// Формирование набора параметров плагина на основе строки конфигурации.
    pub(super) fn from_params(p: *const c_char) -> Self {
        let default = Params::default();
        if p.is_null() {
            return default;
        }

        let mut radius = default.radius;
        let mut iterations = default.iterations;

        let p_str = unsafe { CStr::from_ptr(p).to_string_lossy().to_lowercase() };
        let params: Vec<&str> = p_str.split(',').map(|s| s.trim()).collect();

        for param in params {
            let Some((key, value)) = param.split_once('=') else {
                continue;
            };

            if key.eq_ignore_ascii_case("radius") {
                radius = value.parse().unwrap_or(DEFAULT_RADIUS)
            }

            if key.eq_ignore_ascii_case("iterations") {
                iterations = value.parse().unwrap_or(DEFAULT_ITERATIONS)
            }
        }

        Self { radius, iterations }
    }
}
