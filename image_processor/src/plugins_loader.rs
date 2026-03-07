//! Взаимодействие с плагинами для обработки изображений.

use anyhow::{Context, Result as AnyhowResult, ensure};
use image::RgbaImage;
use libloading::{Library, Symbol, library_filename};
use std::{
    ffi::{CString, c_char},
    path::{Path, PathBuf},
};

/// Название функции обработки изображения (process_image).
const FUNC_PROCESS_IMAGE: &str = "process_image";
/// Тип для выгрузки функции из библиотеки C.
type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);

/// Загрузчик плагина с методами для доступа к их функциям.
pub(crate) struct PluginLoader {
    /// Загруженная библиотека с плагином.
    lib: Library,
}

impl PluginLoader {
    /// Создать загрузчик и открыть динамическую библиотеку плагина.
    pub(crate) fn new(plugin_name: &str, plugins_dir: &Path) -> AnyhowResult<Self> {
        let plugin_file = Self::get_full_plugin_path(plugin_name, plugins_dir)?;

        // SAFETY: Загружаем библиотеку по вычисленному пути.
        let lib = unsafe {
            Library::new(&plugin_file).with_context(|| {
                format!("Не удаётся загрузить плагин: `{}`", plugin_file.display())
            })?
        };

        Ok(Self { lib })
    }

    /// Сформировать полный путь к плагину и проверить, что файл существует.
    fn get_full_plugin_path(plugin_name: &str, plugins_dir: &Path) -> AnyhowResult<PathBuf> {
        let lib_name = library_filename(format!("{plugin_name}_plugin"));
        let path = plugins_dir.join(lib_name);

        ensure!(
            path.is_file(),
            format!("Плагин не найден: `{}`", path.display())
        );

        Ok(path)
    }

    /// Вызвать `process_image` из плагина.
    pub(crate) fn process_image(&self, image: &mut RgbaImage, params: &str) -> AnyhowResult<()> {
        let func_name = FUNC_PROCESS_IMAGE.as_bytes();

        // SAFETY: Ищем функцию в загруженной библиотеке.
        let func: Symbol<ProcessImageFn> = unsafe {
            self.lib
                .get(func_name)
                .with_context(|| format!("Ошибка загрузки функции `{FUNC_PROCESS_IMAGE}`"))?
        };

        let param_c =
            CString::new(params).with_context(|| "Параметры содержат внутренний NULL-байт")?;

        let width = image.width();
        let height = image.height();
        let data = image.as_mut();

        // Проверим переполнение (согласно ТЗ).
        Self::check_overload_buffer(width, height, data.len())?;

        println!("Processing image...");
        // SAFETY: Вызываем функцию с ABI и сигнатурой, соответствующей `ProcessImageFn`.
        unsafe { func(width, height, data.as_mut_ptr(), param_c.as_ptr()) };
        println!("Completed!");

        Ok(())
    }

    /// Проверить, что длина буфера вычислена правильно (width * height * 4).
    fn check_overload_buffer(width: u32, height: u32, data_len: usize) -> AnyhowResult<()> {
        let expect = (width as usize)
            .checked_mul(height as usize)
            .and_then(|v| v.checked_mul(4))
            .with_context(|| "Переполнение при расчёте размера буфера")?;

        ensure!(
            data_len == expect,
            "Некорректный размер RGBA-буфера: actual={data_len}, expected={expect}, w={width}, h={height}"
        );
        Ok(())
    }
}
