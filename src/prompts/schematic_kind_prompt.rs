use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::cli::schematic_kind::SchematicKind;

pub fn ask() -> Result<SchematicKind> {
    let options = SchematicKind::all();
    let labels: Vec<&str> = options.iter().map(|k| k.label()).collect();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to generate?")
        .items(&labels)
        .default(0)
        .interact()
        .context("schematic kind prompt failed")?;

    Ok(options[index])
}
