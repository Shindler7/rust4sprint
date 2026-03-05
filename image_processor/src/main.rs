//! CLI-приложение для обработки изображений с поддержкой динамически
//! подключаемых плагинов через FFI.

mod cli;
mod config;
mod images;
mod plugins_loader;

use crate::{images::get_as_rgba8, plugins_loader::PluginLoader};
use anyhow::Result as AnyhowResult;
use cli::CliArgParser;

fn main() -> AnyhowResult<()> {
    let cli_args = CliArgParser::get_args();

    let rgba8_img = get_as_rgba8(&cli_args.input)?;
    let plug_loader = PluginLoader::new(&cli_args.plugin_name(), &cli_args.get_plugin_path())?;

    let _update_rgba8_img = plug_loader.process_image(&rgba8_img)?;

    Ok(())
}
