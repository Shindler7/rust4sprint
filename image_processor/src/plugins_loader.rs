//! Взаимодействие с плагинами для обработки изображений.

use crate::config::DEFAULT_PLUGIN_EXT;
use anyhow::{ensure, Context, Result as AnyhowResult};
use image::RgbaImage;
use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};

/// Название функции обработки изображения (process_image).
const FUNC_PROCESS_IMAGE: &str = "process_image";
type ProcessImageFn = unsafe extern "C" fn();

/// Загрузчик плагина с методами для доступа к их функциям.
pub(crate) struct PluginLoader {
    /// Загруженная библиотека с плагином.
    lib: Library,
}

impl PluginLoader {
    /// Создать загрузчик и открыть динамическую библиотеку плагина.
    pub(crate) fn new(plugin_name: &str, plugins_dir: &Path) -> AnyhowResult<Self> {
        let plugin_file = Self::get_full_plugin_path(plugin_name, plugins_dir, None)?;

        // SAFETY: Загружаем библиотеку по вычисленному пути.
        let lib = unsafe {
            Library::new(&plugin_file).with_context(|| {
                format!("Не удаётся загрузить плагин: `{}`", plugin_file.display())
            })?
        };

        Ok(Self { lib })
    }

    /// Сформировать полный путь к плагину и проверить, что файл существует.
    ///
    /// Имя: `<plugin>_plugin.<ext>`, где `ext` по умолчанию
    /// [`DEFAULT_PLUGIN_EXT`].
    fn get_full_plugin_path(
        plugin_name: &str,
        plugins_dir: &Path,
        ext: Option<&str>,
    ) -> AnyhowResult<PathBuf> {
        let ext = ext.filter(|s| !s.is_empty()).unwrap_or(DEFAULT_PLUGIN_EXT);
        let path = plugins_dir.join(format!("{plugin_name}_plugin.{ext}"));

        ensure!(
            path.is_file(),
            format!("Плагин не найден: `{}`", path.display())
        );

        Ok(path)
    }

    /// Вызвать `process_image` из плагина.
    pub(crate) fn process_image(&self, image: &RgbaImage) -> AnyhowResult<RgbaImage> {
        let func_name = FUNC_PROCESS_IMAGE.as_bytes();

        // SAFETY: Ищем функцию в загруженной библиотеке.
        let func: Symbol<ProcessImageFn> = unsafe {
            self.lib
                .get(func_name)
                .with_context(|| format!("Ошибка загрузки функции `{FUNC_PROCESS_IMAGE}`"))?
        };

        // SAFETY: Вызываем функцию с ABI и сигнатурой, соответствующей `ProcessImageFn`.
        unsafe { func() };

        Ok(image.clone())
    }
}
