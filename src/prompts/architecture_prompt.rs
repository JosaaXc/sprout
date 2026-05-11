use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::context::architecture::Architecture;

pub fn ask() -> Result<Architecture> {
    let options = Architecture::all();
    let labels: Vec<&str> = options.iter().map(|a| a.label()).collect();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which architecture do you prefer?")
        .items(&labels)
        .default(0)
        .interact()
        .context("architecture prompt failed")?;

    Ok(options[index])
}
