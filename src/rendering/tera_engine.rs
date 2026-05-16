use anyhow::{Context, Result};
use console::style;
use serde::Serialize;
use std::path::Path;
use tera::Tera;

use crate::rendering::template_loader;

pub struct TeraEngine {
    inner: Tera,
    overrides: Vec<String>,
}

impl TeraEngine {
    pub fn with_embedded_templates() -> Result<Self> {
        let mut tera = Tera::default();
        for name in template_loader::all_template_names() {
            let src = template_loader::load(&name)?;
            tera.add_raw_template(&name, &src)
                .with_context(|| format!("failed to register template {name}"))?;
        }
        Ok(Self {
            inner: tera,
            overrides: Vec::new(),
        })
    }

    pub fn with_overrides(project_root: &Path) -> Result<Self> {
        let mut engine = Self::with_embedded_templates()?;
        let override_dir = project_root.join(".sprout").join("templates");
        if override_dir.is_dir() {
            engine.load_overrides(&override_dir)?;
        }
        Ok(engine)
    }

    fn load_overrides(&mut self, dir: &Path) -> Result<()> {
        for entry in walkdir::WalkDir::new(dir).into_iter().flatten() {
            if !entry.file_type().is_file() {
                continue;
            }
            let rel = entry
                .path()
                .strip_prefix(dir)
                .with_context(|| format!("strip_prefix failed for {}", entry.path().display()))?
                .to_string_lossy()
                .replace('\\', "/");
            let contents = std::fs::read_to_string(entry.path())
                .with_context(|| format!("reading override template {}", entry.path().display()))?;
            self.inner
                .add_raw_template(&rel, &contents)
                .with_context(|| format!("registering override template {rel}"))?;
            self.overrides.push(rel);
        }
        Ok(())
    }

    pub fn announce_overrides(&self) {
        if self.overrides.is_empty() {
            return;
        }
        for name in &self.overrides {
            println!(
                "  {} Using local override: {}",
                style("OVERRIDE").magenta().bold(),
                style(format!(".sprout/templates/{name}")).bold()
            );
        }
    }

    pub fn render<T: Serialize>(&self, template: &str, ctx: &T) -> Result<String> {
        let tera_ctx = tera::Context::from_serialize(ctx)?;
        self.inner
            .render(template, &tera_ctx)
            .with_context(|| format!("failed to render template {template}"))
    }

    pub fn render_with<T: Serialize, E: Serialize>(
        &self,
        template: &str,
        ctx: &T,
        extras: &E,
    ) -> Result<String> {
        let mut tera_ctx = tera::Context::from_serialize(ctx)?;
        let extras_ctx = tera::Context::from_serialize(extras)?;
        tera_ctx.extend(extras_ctx);
        self.inner
            .render(template, &tera_ctx)
            .with_context(|| format!("failed to render template {template}"))
    }
}
