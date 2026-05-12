use anyhow::{anyhow, Result};

use crate::cli::schematic_kind::SchematicKind;
use crate::context::architecture::Architecture;
use crate::context::dto_style::DtoStyle;
use crate::context::persistence::Persistence;

pub mod architecture_prompt;
pub mod dto_style_prompt;
pub mod name_prompt;
pub mod persistence_prompt;
pub mod schematic_kind_prompt;

pub trait InteractivePrompter {
    fn ask_schematic_kind(&self) -> Result<SchematicKind>;
    fn ask_name(&self, kind: SchematicKind) -> Result<String>;
    fn ask_architecture(&self) -> Result<Architecture>;
    fn ask_dto_style(&self) -> Result<DtoStyle>;
    fn ask_persistence(&self) -> Result<Persistence>;
}

pub struct DialoguerPrompter;

impl DialoguerPrompter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DialoguerPrompter {
    fn default() -> Self {
        Self::new()
    }
}

fn non_interactive() -> bool {
    std::env::var("SPROUT_NON_INTERACTIVE").is_ok()
}

impl InteractivePrompter for DialoguerPrompter {
    fn ask_schematic_kind(&self) -> Result<SchematicKind> {
        if let Ok(raw) = std::env::var("SPROUT_SCHEMATIC_KIND") {
            return parse_schematic_kind(&raw);
        }
        if non_interactive() {
            return Err(anyhow!(
                "no schematic kind provided. In non-interactive mode pass it as a positional argument (e.g. `sprout g resource user`) or set SPROUT_SCHEMATIC_KIND"
            ));
        }
        schematic_kind_prompt::ask()
    }

    fn ask_name(&self, kind: SchematicKind) -> Result<String> {
        if let Ok(name) = std::env::var("SPROUT_NAME") {
            let trimmed = name.trim().to_string();
            if trimmed.is_empty() {
                return Err(anyhow!("SPROUT_NAME is set but empty"));
            }
            return Ok(trimmed);
        }
        if non_interactive() {
            return Err(anyhow!(
                "no name provided. In non-interactive mode pass it as a positional argument (e.g. `sprout g resource user`) or set SPROUT_NAME"
            ));
        }
        name_prompt::ask(kind)
    }

    fn ask_architecture(&self) -> Result<Architecture> {
        if let Ok(arch) = std::env::var("SPROUT_ARCHITECTURE") {
            return match arch.as_str() {
                "modular" => Ok(Architecture::Modular),
                "layered" => Ok(Architecture::Layered),
                "hexagonal" => Ok(Architecture::Hexagonal),
                other => Err(anyhow!(
                    "unknown SPROUT_ARCHITECTURE '{other}', expected one of: modular, layered, hexagonal"
                )),
            };
        }
        if non_interactive() {
            return Ok(Architecture::default());
        }
        architecture_prompt::ask()
    }

    fn ask_dto_style(&self) -> Result<DtoStyle> {
        if let Ok(style) = std::env::var("SPROUT_DTO_STYLE") {
            return match style.as_str() {
                "record" => Ok(DtoStyle::Record),
                "class" => Ok(DtoStyle::Class),
                other => Err(anyhow!(
                    "unknown SPROUT_DTO_STYLE '{other}', expected one of: record, class"
                )),
            };
        }
        if non_interactive() {
            return Ok(DtoStyle::default());
        }
        dto_style_prompt::ask()
    }

    fn ask_persistence(&self) -> Result<Persistence> {
        if let Ok(pers) = std::env::var("SPROUT_PERSISTENCE") {
            return match pers.as_str() {
                "jpa" | "jpa-relational" => Ok(Persistence::JpaRelational),
                "mongo" | "mongodb" => Ok(Persistence::MongoDb),
                other => Err(anyhow!(
                    "unknown SPROUT_PERSISTENCE '{other}', expected one of: jpa, mongo"
                )),
            };
        }
        if non_interactive() {
            return Ok(Persistence::default());
        }
        persistence_prompt::ask()
    }
}

fn parse_schematic_kind(raw: &str) -> Result<SchematicKind> {
    match raw {
        "resource" => Ok(SchematicKind::Resource),
        "service" => Ok(SchematicKind::Service),
        "controller" => Ok(SchematicKind::Controller),
        "entity" => Ok(SchematicKind::Entity),
        "repository" => Ok(SchematicKind::Repository),
        "dto" => Ok(SchematicKind::Dto),
        "mapper" => Ok(SchematicKind::Mapper),
        other => Err(anyhow!(
            "unknown SPROUT_SCHEMATIC_KIND '{other}', expected one of: resource, service, controller, entity, repository, dto, mapper"
        )),
    }
}
