use anyhow::{Context, Result as AnyhowResult};
use std::{env, path::PathBuf};

fn main() -> AnyhowResult<()> {
    let bindings = bindgen::builder()
        .header("src/mirror_plugin.h")
        .generate()
        .with_context(|| "failed to generate bindings")?;

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(out_dir).join("mirror_plugin.rs");
    bindings
        .write_to_file(out_path)
        .with_context(|| "failed to write bindings")?;

    cc::Build::new()
        .file("src/mirror_plugin.c")
        .compile("mirror_plugin");

    Ok(())
}
