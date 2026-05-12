use anyhow::Result;
use clap::Subcommand;

use crate::context::generation_context::GenerationContext;
use crate::prompts::InteractivePrompter;
use crate::workspace::project_detector::ProjectContext;

#[derive(Debug, Clone, Subcommand)]
pub enum SchematicKind {
    #[command(about = "Full CRUD: entity, dto, mapper, service, controller")]
    Resource { name: String },
    #[command(about = "Service interface + @Service impl with @RequiredArgsConstructor")]
    Service { name: String },
    #[command(about = "@RestController with @ResponseStatus endpoints")]
    Controller { name: String },
    #[command(about = "JPA or Mongo entity, decorated with Lombok")]
    Entity { name: String },
    #[command(about = "DTO (Java record or class) with Bean Validation")]
    Dto { name: String },
    #[command(about = "MapStruct mapper bound to the Spring component model")]
    Mapper { name: String },
    #[command(about = "Spring Data repository (JPA or Mongo) for the entity")]
    Repository { name: String },
}

impl SchematicKind {
    pub fn name(&self) -> &str {
        match self {
            Self::Resource { name }
            | Self::Service { name }
            | Self::Controller { name }
            | Self::Entity { name }
            | Self::Dto { name }
            | Self::Mapper { name }
            | Self::Repository { name } => name,
        }
    }

    pub fn needs_dto_style(&self) -> bool {
        matches!(self, Self::Resource { .. } | Self::Dto { .. })
    }

    pub fn needs_persistence(&self) -> bool {
        matches!(
            self,
            Self::Resource { .. } | Self::Entity { .. } | Self::Repository { .. }
        )
    }

    pub fn collect_context(
        &self,
        raw_name: &str,
        project: &ProjectContext,
        prompter: &dyn InteractivePrompter,
        skip_test: bool,
    ) -> Result<GenerationContext> {
        GenerationContext::build(self, raw_name, project, prompter, skip_test)
    }
}
