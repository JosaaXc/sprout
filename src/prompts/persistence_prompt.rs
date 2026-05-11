use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::context::persistence::Persistence;

pub fn ask() -> Result<Persistence> {
    let options = Persistence::all();
    let labels: Vec<&str> = options.iter().map(|p| p.label()).collect();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of database will you use for this resource?")
        .items(&labels)
        .default(0)
        .interact()
        .context("persistence prompt failed")?;

    Ok(options[index])
}
