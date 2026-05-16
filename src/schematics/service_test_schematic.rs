use anyhow::Result;

use crate::context::architecture::ArtifactKind;
use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{artifact_path, Schematic, SchematicOutput};

pub struct ServiceTestSchematic;

impl Schematic for ServiceTestSchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let contents = engine.render("test/service_test.java.tera", ctx)?;
        let sub_path = ctx.architecture.path_for(
            &ctx.name.kebab,
            ArtifactKind::ServiceTest,
            &ctx.conventions,
        );

        Ok(vec![SchematicOutput {
            relative_path: artifact_path(
                &sub_path,
                &format!("{}ServiceTest.java", ctx.name.pascal),
            ),
            contents,
            is_test: true,
        }])
    }
}
