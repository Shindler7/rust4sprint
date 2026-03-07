//! Инфраструктура парсинга аргументов командной строки.
//!
//! Создано с помощью `clap`.

use crate::tools::get_workspace_root;
use anyhow::{Context, Result as AnyhowResult};
use clap::Parser;
use image::ImageFormat;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgParser {
    /// Path to source PNG image.
    #[clap(short, long, value_parser=validate_exists_png_file)]
    pub(crate) input: PathBuf,

    /// Path where the processed image will be saved.
    #[clap(short, long, value_parser=validate_exists_dir)]
    pub(crate) output: PathBuf,

    /// Path to processing parameters file.
    #[clap(short, long, value_parser=validate_exists_file)]
    params: PathBuf,

    /// Path to the directory containing the plugin (default: `target/debug`).
    #[clap(short = 'P', long, value_parser=validate_exists_dir)]
    plugin_path: Option<PathBuf>,

    /// Plugin name (for example: mirror).
    #[clap(short = 'n', long)]
    plugin: String,
}

/// Валидатор для [`CliArgParser`]: проверка существования директории.
fn validate_exists_dir(dir: &str) -> Result<PathBuf, String> {
    let validate_dir = PathBuf::from(dir);
    if validate_dir.is_dir() {
        Ok(validate_dir)
    } else {
        Err("Directory doesn't exist or permission denied".to_string())
    }
}

/// Валидатор для [`CliArgParser`]: проверка существования файла.
fn validate_exists_file(file_path: &str) -> Result<PathBuf, String> {
    let file = PathBuf::from(file_path);
    if file.try_exists().map_err(|e| e.to_string())? {
        Ok(file)
    } else {
        Err("File not found".to_string())
    }
}

/// Валидатор для [`CliArgParser`]: проверка существования png-файла, а также
/// соответствия расширения (подлинность формату не проверяется).
fn validate_exists_png_file(file_path: &str) -> Result<PathBuf, String> {
    let file = validate_exists_file(file_path)?;
    let is_png = ImageFormat::from_path(&file)
        .map_err(|e| e.to_string())?
        .eq(&ImageFormat::Png);

    if is_png {
        Ok(file)
    } else {
        Err("Only PNG files are allowed".to_string())
    }
}

impl CliArgParser {
    /// Получить нормализованные параметры из командной строки.
    pub(crate) fn get_args() -> Self {
        Self::parse()
    }

    /// Предоставить путь к директории с плагином.
    ///
    /// Если `plugin_path` отсутствует:
    /// - в `debug` формируется путь к `target/debug`
    /// - в `release` к директории расположения запускаемого файла
    pub(crate) fn get_plugin_path(&self) -> PathBuf {
        self.plugin_path
            .clone()
            .unwrap_or_else(Self::get_default_plugin_path)
    }

    /// Формирование пути по умолчанию для плагинов.
    fn get_default_plugin_path() -> PathBuf {
        if cfg!(debug_assertions) {
            get_workspace_root().join("target").join("debug")
        } else {
            get_workspace_root()
        }
    }

    /// Вернуть имя плагина в строковом представлении.
    pub(crate) fn plugin_name(&self) -> String {
        self.plugin.clone()
    }

    /// Загрузить содержимое файла параметров.
    pub(crate) fn read_params(&self) -> AnyhowResult<String> {
        let mut data = String::new();
        let mut file = File::open(&self.params).with_context(|| {
            format!(
                "Ошибка доступа к файлу параметров: '{}'",
                self.params.display()
            )
        })?;
        file.read_to_string(&mut data)
            .with_context(|| "Не удалось прочитать параметры")?;

        Ok(data)
    }
}
