//! Инфраструктура парсинга аргументов командной строки.
//!
//! Создано с помощью `clap`.

use clap::{Parser, ValueEnum};
use std::{
    fmt::{Display, Result as FmtResult},
    path::PathBuf,
};

/// Supported plugins.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Plugins {
    /// Mirror image flip.
    Mirror,
}

impl Display for Plugins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self {
            Plugins::Mirror => write!(f, "mirror"),
        }
    }
}

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
    pub(crate) params: PathBuf,

    /// Path to the directory containing the plugin (default: `target/debug`).
    #[clap(short = 'P', long, value_parser=validate_exists_dir)]
    plugin_path: Option<PathBuf>,

    /// Plugin name (dynamic library name without extension, e.g., invert).
    #[clap(value_enum)]
    plugin: Plugins,
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
    let is_png = file
        .extension()
        .and_then(|s| s.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("png"));

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
    /// Если `plugin_path` отсутствует, формируется путь к `target/debug`.
    pub(crate) fn get_plugin_path(&self) -> PathBuf {
        self.plugin_path.clone().unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("debug")
        })
    }
}
