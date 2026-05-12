use anyhow::Result;

use crate::context::architecture::Architecture;
use crate::context::dto_style::DtoStyle;
use crate::context::persistence::Persistence;

pub mod architecture_prompt;
pub mod dto_style_prompt;
pub mod persistence_prompt;

pub trait InteractivePrompter {
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

impl InteractivePrompter for DialoguerPrompter {
    fn ask_architecture(&self) -> Result<Architecture> {
        if let Ok(arch) = std::env::var("SPROUT_ARCHITECTURE") {
            return match arch.as_str() {
                "layered" => Ok(Architecture::Layered),
                "hexagonal" => Ok(Architecture::Hexagonal),
                _ => Ok(Architecture::Modular),
            };
        }
        if std::env::var("SPROUT_NON_INTERACTIVE").is_ok() {
            return Ok(Architecture::default());
        }
        architecture_prompt::ask()
    }

    fn ask_dto_style(&self) -> Result<DtoStyle> {
        if let Ok(style) = std::env::var("SPROUT_DTO_STYLE") {
            return match style.as_str() {
                "class" => Ok(DtoStyle::Class),
                _ => Ok(DtoStyle::Record),
            };
        }
        if std::env::var("SPROUT_NON_INTERACTIVE").is_ok() {
            return Ok(DtoStyle::default());
        }
        dto_style_prompt::ask()
    }

    fn ask_persistence(&self) -> Result<Persistence> {
        if let Ok(pers) = std::env::var("SPROUT_PERSISTENCE") {
            return match pers.as_str() {
                "mongo" => Ok(Persistence::MongoDb),
                _ => Ok(Persistence::JpaRelational),
            };
        }
        if std::env::var("SPROUT_NON_INTERACTIVE").is_ok() {
            return Ok(Persistence::default());
        }
        persistence_prompt::ask()
    }
}
