//! CLI-приложение для обработки изображений с поддержкой динамически
//! подключаемых плагинов через FFI.

mod cli;
use anyhow::{Context, Result as AnyhowResult};
use cli::CliArgParser;
use image::ImageReader;

fn main() -> AnyhowResult<()> {
    let cli_args = CliArgParser::get_args();

    let image = ImageReader::open(&cli_args.input)
        .with_context(|| {
            format!(
                "Ошибка чтения файла изображения: {}",
                &cli_args.input.to_string_lossy()
            )
        })?
        .decode()
        .with_context(|| "Неподдерживаемый формат изображения")?;

    Ok(())
}
