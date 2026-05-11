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
        architecture_prompt::ask()
    }

    fn ask_dto_style(&self) -> Result<DtoStyle> {
        dto_style_prompt::ask()
    }

    fn ask_persistence(&self) -> Result<Persistence> {
        persistence_prompt::ask()
    }
}
