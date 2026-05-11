use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::context::dto_style::DtoStyle;

pub fn ask() -> Result<DtoStyle> {
    let options = DtoStyle::all();
    let labels: Vec<&str> = options.iter().map(|d| d.label()).collect();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to generate the DTOs?")
        .items(&labels)
        .default(0)
        .interact()
        .context("dto style prompt failed")?;

    Ok(options[index])
}
