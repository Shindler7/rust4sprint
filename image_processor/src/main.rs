//! CLI-приложение для обработки изображений с поддержкой динамически
//! подключаемых плагинов через FFI.

mod cli;
use anyhow::Result as AnyhowResult;
use cli::CliArgParser;

fn main() -> AnyhowResult<()> {
    let _cli_args = CliArgParser::get_args();

    Ok(())
}
