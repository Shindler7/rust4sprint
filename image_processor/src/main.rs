//! CLI-приложение для обработки изображений с поддержкой динамически
//! подключаемых плагинов через FFI.

mod cli;
mod config;
mod images;
mod plugins_loader;
mod tools;

use crate::{
    images::{get_as_rgba8, save_rgba8},
    plugins_loader::PluginLoader,
    tools::FileExt,
};
use anyhow::Result as AnyhowResult;
use cli::CliArgParser;

fn main() -> AnyhowResult<()> {
    let cli_args = CliArgParser::get_args();

    let file_ext = FileExt::new(&cli_args.input)?;
    let mut rgba8_img = get_as_rgba8(&cli_args.input)?;

    let plug_loader = PluginLoader::new(&cli_args.plugin_name(), &cli_args.get_plugin_path())?;
    plug_loader.process_image(&mut rgba8_img, &cli_args.read_params()?)?;

    let target_file = file_ext.nhe_with_path(&cli_args.output);
    save_rgba8(&target_file, &rgba8_img)?;

    Ok(())
}
