use anyhow::Result;

use crate::context::architecture::ArtifactKind;
use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{artifact_path, Schematic, SchematicOutput};

pub struct DtoSchematic;

impl Schematic for DtoSchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let sub_path = ctx
            .architecture
            .path_for(&ctx.name.kebab, ArtifactKind::Dto);
        let request = engine.render(ctx.dto_style.request_template(), ctx)?;
        let response = engine.render(ctx.dto_style.response_template(), ctx)?;
        Ok(vec![
            SchematicOutput {
                relative_path: artifact_path(
                    &sub_path,
                    &format!("{}Request.java", ctx.name.pascal),
                ),
                contents: request,
                is_test: false,
            },
            SchematicOutput {
                relative_path: artifact_path(
                    &sub_path,
                    &format!("{}Response.java", ctx.name.pascal),
                ),
                contents: response,
                is_test: false,
            },
        ])
    }
}
