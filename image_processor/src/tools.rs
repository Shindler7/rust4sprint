//! Поддерживающие методы (утилиты) для приложения.

use anyhow::{Context, Result as AnyhowResult};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::Path,
    time::Instant,
};

/// Обработчик файловых путей.
pub(crate) struct FileExt {
    /// Имя файла без расширения.
    stem: String,
    /// Расширение файла.
    ext: String,
}

impl FileExt {
    /// Создать объект.
    pub(crate) fn new(file: &Path) -> AnyhowResult<Self> {
        let stem = file
            .file_stem()
            .with_context(|| format!("Не найдено имя файла: '{}'", file.display()))
            .and_then(|s| os_str_to_string(s, "Имя файла (stem)", file))?;

        let ext = file
            .extension()
            .with_context(|| format!("Отсутствует расширение у файла: `{}`", file.display()))
            .and_then(|s| os_str_to_string(s, "Расширение файла", file))?;

        Ok(Self { stem, ext })
    }

    /// Создать новое имя файла с дополнением случайной хеш-строкой.
    ///
    /// `input.txt` => `input_3acd09b9b0bb73c5.txt`
    pub(crate) fn name_hash_ext(&self) -> String {
        let hash = Self::short_id();
        format!("{}_{}.{}", self.stem, hash, self.ext)
    }

    /// Создать полный путь с новым именем файла с хеш-строкой.
    pub(crate) fn nhe_with_path(&self, target_dir: &Path) -> PathBuf {
        target_dir.join(self.name_hash_ext())
    }

    /// Создать случайную хеш-строку.
    fn short_id() -> String {
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Helper для преобразования `os_str` в `String`.
fn os_str_to_string(s: &OsStr, what: &str, file: &Path) -> AnyhowResult<String> {
    s.to_str()
        .map(str::to_owned)
        .with_context(|| format!("{what} не соответствует UTF-8: '{}'", file.display()))
}

/// Предоставить родительский каталог проекта.
///
/// Для `debug` это будет директория расположения `Cargo.toml`, а для `release`
/// расположение скомпилированного файла.
#[cfg(debug_assertions)]
pub(crate) fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(not(debug_assertions))]
pub(crate) fn get_project_root() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .expect("Не удалось определить путь к исполняемому файлу")
}

/// Предоставить корневую директорию всего проекта.
///
/// В зависимости от статуса проекта предоставляет путь к корневой директории
/// `workspace`, а для `release` к месту расположения скомпилированного файла,
/// что также является корневым путём.
///
/// Вызывает панику при неудачах определения путей.
pub(crate) fn get_workspace_root() -> PathBuf {
    let project_root = get_project_root();
    if cfg!(debug_assertions) {
        project_root
            .parent()
            .expect("Не удалось получить родительский каталог workspace")
            .to_path_buf()
    } else {
        project_root.to_path_buf()
    }
}
