use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src/cmd/webui/web/dist"]
pub struct Assets;
