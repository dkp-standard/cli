use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let src_dist = Path::new(&manifest_dir).join("src/cmd/webui/web/dist");

    let embed_dir = if src_dist.exists()
        && src_dist
            .read_dir()
            .is_ok_and(|mut d| d.next().is_some())
    {
        // Real Svelte build present — embed it.
        src_dist.to_string_lossy().into_owned()
    } else {
        // No dist yet (cargo publish verification, fresh checkout).
        // Create a stub in OUT_DIR so RustEmbed always has a folder.
        let stub = Path::new(&out_dir).join("webui_stub");
        fs::create_dir_all(&stub).expect("failed to create webui stub dir");
        fs::write(
            stub.join("index.html"),
            "<!doctype html><html><body>Run `pnpm build` in src/cmd/webui/web/ first.</body></html>",
        )
        .expect("failed to write stub index.html");
        stub.to_string_lossy().into_owned()
    };

    // Generate embed.rs with the resolved literal path.
    let embed_rs = format!(
        r#"use rust_embed::Embed;

#[derive(Embed)]
#[folder = "{embed_dir}"]
pub struct Assets;
"#
    );
    fs::write(Path::new(&out_dir).join("embed.rs"), embed_rs).expect("failed to write embed.rs");

    println!("cargo:rerun-if-changed=src/cmd/webui/web/dist");
}
