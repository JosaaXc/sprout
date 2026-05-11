use anyhow::{Context, Result};
use serde::Serialize;
use tera::Tera;

use crate::rendering::template_loader;

pub struct TeraEngine {
    inner: Tera,
}

impl TeraEngine {
    pub fn with_embedded_templates() -> Result<Self> {
        let mut tera = Tera::default();
        for name in template_loader::all_template_names() {
            let src = template_loader::load(&name)?;
            tera.add_raw_template(&name, &src)
                .with_context(|| format!("failed to register template {name}"))?;
        }
        Ok(Self { inner: tera })
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
