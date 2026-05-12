use anyhow::{anyhow, Context, Result};
use dialoguer::{theme::ColorfulTheme, Input};

use crate::cli::schematic_kind::SchematicKind;

pub fn ask(kind: SchematicKind) -> Result<String> {
    let prompt_label = match kind {
        SchematicKind::Resource => "Resource name",
        SchematicKind::Service => "Service name",
        SchematicKind::Controller => "Controller name",
        SchematicKind::Entity => "Entity name",
        SchematicKind::Repository => "Repository name",
        SchematicKind::Dto => "DTO name",
        SchematicKind::Mapper => "Mapper name",
    };

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("{prompt_label} (kebab-case or PascalCase, e.g. user-account)"))
        .validate_with(|input: &String| -> std::result::Result<(), &str> {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                Err("Name cannot be empty")
            } else if !trimmed
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                Err("Name may only contain letters, digits, '-' and '_'")
            } else if !trimmed.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
                Err("Name must start with a letter")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .context("name prompt failed")?;

    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err(anyhow!("empty name after trim"));
    }
    Ok(trimmed)
}
