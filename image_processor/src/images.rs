//! Функции и методы для работы с изображениями.

use anyhow::{Context, Result as AnyhowResult};
use image::{DynamicImage, ImageFormat, ImageReader, RgbaImage};
use std::path::Path;

/// Загрузить изображение и вернуть в формате `rgba8`.
///
/// ## Args:
///
/// - `path_to_img` — путь к исходному изображению
pub(crate) fn get_as_rgba8(path_to_img: &Path) -> AnyhowResult<RgbaImage> {
    Ok(get_image(path_to_img)?.to_rgba8())
}

/// Сохранить изображение `rgba8` в файл.
pub(crate) fn save_rgba8(target_file: &Path, img: &RgbaImage) -> AnyhowResult<()> {
    let format = ImageFormat::Png;

    img.save_with_format(target_file, format)
        .with_context(|| format!("Ошибка при сохранении файла: {}", target_file.display()))?;

    Ok(())
}

/// Загрузить изображение из файла и вернуть в виде объекта.
///
/// ## Args:
///
/// - `path_to_img` — путь к исходному изображению
fn get_image(path_to_img: &Path) -> AnyhowResult<DynamicImage> {
    ImageReader::open(path_to_img)
        .with_context(|| format!("Ошибка чтения файла изображения: {}", path_to_img.display()))?
        .decode()
        .with_context(|| "Неподдерживаемый формат изображения")
}
