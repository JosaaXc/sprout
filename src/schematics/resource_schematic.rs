use anyhow::Result;

use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::{
    controller_schematic::ControllerSchematic, dto_schematic::DtoSchematic,
    entity_schematic::EntitySchematic, mapper_schematic::MapperSchematic,
    repository_schematic::RepositorySchematic, service_schematic::ServiceSchematic, Schematic,
    SchematicOutput,
};

pub struct ResourceSchematic;

impl Schematic for ResourceSchematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>> {
        let parts: Vec<Box<dyn Schematic>> = vec![
            Box::new(EntitySchematic),
            Box::new(RepositorySchematic),
            Box::new(DtoSchematic),
            Box::new(MapperSchematic),
            Box::new(ServiceSchematic),
            Box::new(ControllerSchematic),
        ];

        let mut outputs = Vec::new();
        for part in parts {
            outputs.extend(part.generate(ctx, engine)?);
        }
        Ok(outputs)
    }
}
