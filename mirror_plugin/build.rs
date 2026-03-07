use anyhow::Result as AnyhowResult;

fn main() -> AnyhowResult<()> {
    println!("cargo:rerun-if-changed=src/mirror_plugin.c");
    println!("cargo:rerun-if-changed=build.rs");

    cc::Build::new()
        .file("src/mirror_plugin.c")
        .compile("mirror_plugin");

    Ok(())
}
