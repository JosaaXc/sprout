use anyhow::Result;

use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{
    controller_schematic::ControllerSchematic, dto_schematic::DtoSchematic,
    entity_schematic::EntitySchematic, mapper_schematic::MapperSchematic,
    repository_schematic::RepositorySchematic, service_schematic::ServiceSchematic,
    service_test_schematic::ServiceTestSchematic, controller_test_schematic::ControllerTestSchematic,
    Schematic, SchematicOutput,
};

pub struct ResourceSchematic;

impl Schematic for ResourceSchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let mut parts: Vec<Box<dyn Schematic>> = vec![
            Box::new(EntitySchematic),
            Box::new(RepositorySchematic),
            Box::new(DtoSchematic),
            Box::new(MapperSchematic),
            Box::new(ServiceSchematic),
            Box::new(ControllerSchematic),
        ];

        if !ctx.skip_test {
            parts.push(Box::new(ServiceTestSchematic));
            parts.push(Box::new(ControllerTestSchematic));
        }

        let mut outputs = Vec::new();
        for part in parts {
            outputs.extend(part.generate(ctx, engine)?);
        }
        Ok(outputs)
    }
}
