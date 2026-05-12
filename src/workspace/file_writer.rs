use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::path::PathBuf;

use crate::schematics::SchematicOutput;

pub trait OverwritePolicy {
    fn allow_overwrite(&self, target_display: &str) -> Result<bool>;
}

pub struct InteractiveOverwritePolicy;

impl OverwritePolicy for InteractiveOverwritePolicy {
    fn allow_overwrite(&self, target_display: &str) -> Result<bool> {
        let prompt = format!(
            "{} File {} already exists. Overwrite?",
            style("⚠️").yellow(),
            style(target_display).bold()
        );
        let ans = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(false)
            .show_default(true)
            .interact()?;
        Ok(ans)
    }
}

pub enum WriteOutcome {
    Created,
    Overwritten,
    Skipped,
}

pub struct DiskFileWriter {
    pub base_path: PathBuf,
    pub test_base_path: Option<PathBuf>,
    pub policy: Box<dyn OverwritePolicy>,
}

impl DiskFileWriter {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            test_base_path: None,
            policy: Box::new(InteractiveOverwritePolicy),
        }
    }

    pub fn with_policy(base_path: PathBuf, policy: Box<dyn OverwritePolicy>) -> Self {
        Self { base_path, test_base_path: None, policy }
    }

    pub fn write(&self, output: SchematicOutput) -> Result<WriteOutcome> {
        let base = if output.is_test {
            self.test_base_path.as_ref().unwrap_or(&self.base_path)
        } else {
            &self.base_path
        };
        let target = base.join(&output.relative_path);
        let display = target.display().to_string();

        if target.exists() {
            if !self.policy.allow_overwrite(&display)? {
                println!("  {} {}", style("SKIP").yellow().bold(), display);
                return Ok(WriteOutcome::Skipped);
            }
            self.persist(&target, &output.contents)?;
            println!("  {} {}", style("UPDATE").cyan().bold(), display);
            return Ok(WriteOutcome::Overwritten);
        }

        self.persist(&target, &output.contents)?;
        println!("  {} {}", style("CREATE").green().bold(), display);
        Ok(WriteOutcome::Created)
    }

    fn persist(&self, target: &std::path::Path, contents: &str) -> Result<()> {
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(target, contents)?;
        Ok(())
    }
}
