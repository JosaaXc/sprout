use anyhow::{anyhow, Result};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct EmbeddedTemplates;

pub fn load(relative: &str) -> Result<String> {
    let file = EmbeddedTemplates::get(relative)
        .ok_or_else(|| anyhow!("template not found: {relative}"))?;
    Ok(std::str::from_utf8(file.data.as_ref())?.to_string())
}

pub fn all_template_names() -> impl Iterator<Item = String> {
    EmbeddedTemplates::iter().map(|c| c.into_owned())
}
