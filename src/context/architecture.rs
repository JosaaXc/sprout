use serde::Serialize;

use crate::workspace::folder_conventions::FolderConventions;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Architecture {
    #[default]
    Modular,
    Layered,
    Hexagonal,
}

impl Architecture {
    pub fn all() -> &'static [Architecture] {
        &[Self::Modular, Self::Layered, Self::Hexagonal]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Modular => "Modular (feature-first)",
            Self::Layered => "Layered (by responsibility)",
            Self::Hexagonal => "Hexagonal (ports & adapters)",
        }
    }

    pub fn path_for(
        &self,
        feature: &str,
        artifact: ArtifactKind,
        conventions: &FolderConventions,
    ) -> String {
        use ArtifactKind::*;
        match self {
            Self::Layered => override_leaf(artifact, conventions),
            Self::Modular => format!("{feature}/{}", override_leaf(artifact, conventions)),
            Self::Hexagonal => match artifact {
                Service | ServiceTest => format!("{feature}/application"),
                Controller | ControllerTest => format!("{feature}/infrastructure/web"),
                Entity => format!("{feature}/domain/model"),
                Dto => format!("{feature}/application/dto"),
                Mapper => format!("{feature}/infrastructure/mapper"),
                Repository => format!("{feature}/infrastructure/persistence"),
            },
        }
    }

    pub fn package_segment_for(
        &self,
        feature: &str,
        artifact: ArtifactKind,
        conventions: &FolderConventions,
    ) -> String {
        self.path_for(feature, artifact, conventions)
            .replace('/', ".")
    }
}

fn override_leaf(artifact: ArtifactKind, conventions: &FolderConventions) -> String {
    conventions
        .folder_for(artifact)
        .map(String::from)
        .unwrap_or_else(|| artifact.folder().to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum ArtifactKind {
    Service,
    Controller,
    Entity,
    Dto,
    Mapper,
    Repository,
    ServiceTest,
    ControllerTest,
}

impl ArtifactKind {
    pub fn folder(&self) -> &'static str {
        match self {
            Self::Service | Self::ServiceTest => "service",
            Self::Controller | Self::ControllerTest => "controller",
            Self::Entity => "entity",
            Self::Dto => "dto",
            Self::Mapper => "mapper",
            Self::Repository => "repository",
        }
    }
}
