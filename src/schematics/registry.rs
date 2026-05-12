use crate::cli::schematic_kind::SchematicKind;
use crate::schematics::{
    controller_schematic::ControllerSchematic, dto_schematic::DtoSchematic,
    entity_schematic::EntitySchematic, mapper_schematic::MapperSchematic,
    repository_schematic::RepositorySchematic, resource_schematic::ResourceSchematic,
    service_schematic::ServiceSchematic, Schematic,
};

pub struct SchematicRegistry;

impl SchematicRegistry {
    pub fn resolve(kind: SchematicKind) -> Box<dyn Schematic> {
        match kind {
            SchematicKind::Resource => Box::new(ResourceSchematic),
            SchematicKind::Service => Box::new(ServiceSchematic),
            SchematicKind::Controller => Box::new(ControllerSchematic),
            SchematicKind::Entity => Box::new(EntitySchematic),
            SchematicKind::Dto => Box::new(DtoSchematic),
            SchematicKind::Mapper => Box::new(MapperSchematic),
            SchematicKind::Repository => Box::new(RepositorySchematic),
        }
    }
}
