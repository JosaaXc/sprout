use anyhow::Result;

use crate::context::architecture::ArtifactKind;
use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{artifact_path, Schematic, SchematicOutput};

pub struct EntitySchematic;

impl Schematic for EntitySchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let sub_path = ctx
            .architecture
            .path_for(&ctx.name.kebab, ArtifactKind::Entity, &ctx.conventions);
        let contents = engine.render(ctx.persistence.entity_template(), ctx)?;
        Ok(vec![SchematicOutput {
            relative_path: artifact_path(&sub_path, &format!("{}.java", ctx.name.pascal)),
            contents,
            is_test: false,
        }])
    }
}
