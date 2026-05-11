use anyhow::Result;

use crate::context::architecture::ArtifactKind;
use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{artifact_path, Schematic, SchematicOutput};

pub struct ControllerSchematic;

impl Schematic for ControllerSchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let sub_path = ctx
            .architecture
            .path_for(&ctx.name.kebab, ArtifactKind::Controller);
        let contents = engine.render("controller/controller.java.tera", ctx)?;
        Ok(vec![SchematicOutput {
            relative_path: artifact_path(
                &sub_path,
                &format!("{}Controller.java", ctx.name.pascal),
            ),
            contents,
        }])
    }
}
