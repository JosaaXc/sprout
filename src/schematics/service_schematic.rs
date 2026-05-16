use anyhow::Result;

use crate::context::architecture::ArtifactKind;
use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{artifact_path, Schematic, SchematicOutput};

pub struct ServiceSchematic;

impl Schematic for ServiceSchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let sub_path = ctx
            .architecture
            .path_for(&ctx.name.kebab, ArtifactKind::Service, &ctx.conventions);

        let interface = engine.render("service/interface.java.tera", ctx)?;
        let implementation = engine.render("service/implementation.java.tera", ctx)?;

        Ok(vec![
            SchematicOutput {
                relative_path: artifact_path(
                    &sub_path,
                    &format!("{}Service.java", ctx.name.pascal),
                ),
                contents: interface,
                is_test: false,
            },
            SchematicOutput {
                relative_path: artifact_path(
                    &sub_path,
                    &format!("{}ServiceImpl.java", ctx.name.pascal),
                ),
                contents: implementation,
                is_test: false,
            },
        ])
    }
}
