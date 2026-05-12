use serde::Serialize;

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

    pub fn path_for(&self, feature: &str, artifact: ArtifactKind) -> String {
        use ArtifactKind::*;
        match (self, artifact) {
            (Self::Layered, Service) | (Self::Layered, ServiceTest) => "service".into(),
            (Self::Layered, Controller) | (Self::Layered, ControllerTest) => "controller".into(),
            (Self::Layered, Entity) => "entity".into(),
            (Self::Layered, Dto) => "dto".into(),
            (Self::Layered, Mapper) => "mapper".into(),
            (Self::Layered, Repository) => "repository".into(),

            (Self::Modular, k) => format!("{feature}/{}", k.folder()),

            (Self::Hexagonal, Service) | (Self::Hexagonal, ServiceTest) => {
                format!("{feature}/application")
            }
            (Self::Hexagonal, Controller) | (Self::Hexagonal, ControllerTest) => {
                format!("{feature}/infrastructure/web")
            }
            (Self::Hexagonal, Entity) => format!("{feature}/domain/model"),
            (Self::Hexagonal, Dto) => format!("{feature}/application/dto"),
            (Self::Hexagonal, Mapper) => format!("{feature}/infrastructure/mapper"),
            (Self::Hexagonal, Repository) => format!("{feature}/infrastructure/persistence"),
        }
    }

    pub fn package_segment_for(&self, feature: &str, artifact: ArtifactKind) -> String {
        self.path_for(feature, artifact).replace('/', ".")
    }
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
