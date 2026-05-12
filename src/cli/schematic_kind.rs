use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "kebab-case")]
pub enum SchematicKind {
    Resource,
    Service,
    Controller,
    Entity,
    Dto,
    Mapper,
    Repository,
}

impl SchematicKind {
    pub fn all() -> &'static [SchematicKind] {
        &[
            Self::Resource,
            Self::Service,
            Self::Controller,
            Self::Entity,
            Self::Repository,
            Self::Dto,
            Self::Mapper,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Resource => "Resource — full CRUD slice (entity, repo, dto, mapper, service, controller)",
            Self::Service => "Service — interface + @Service impl",
            Self::Controller => "Controller — @RestController with @ResponseStatus endpoints",
            Self::Entity => "Entity — JPA or Mongo, Lombok-decorated",
            Self::Repository => "Repository — Spring Data interface",
            Self::Dto => "DTO — record or class with Bean Validation",
            Self::Mapper => "Mapper — MapStruct (componentModel = \"spring\")",
        }
    }

    pub fn needs_dto_style(&self) -> bool {
        matches!(self, Self::Resource | Self::Dto)
    }

    pub fn needs_persistence(&self) -> bool {
        matches!(self, Self::Resource | Self::Entity | Self::Repository)
    }
}
