use anyhow::Result;
use std::path::PathBuf;

use crate::context::generation_context::GenerationContext;
use crate::rendering::tera_engine::TeraEngine;

pub mod controller_schematic;
pub mod dto_schematic;
pub mod entity_schematic;
pub mod mapper_schematic;
pub mod registry;
pub mod repository_schematic;
pub mod resource_schematic;
pub mod service_schematic;
pub mod service_test_schematic;
pub mod controller_test_schematic;

pub struct SchematicOutput {
    pub relative_path: PathBuf,
    pub contents: String,
    pub is_test: bool,
}

pub trait Schematic {
    fn generate(
        &self,
        ctx: &GenerationContext,
        engine: &TeraEngine,
    ) -> Result<Vec<SchematicOutput>>;
}

pub(crate) fn artifact_path(sub_path: &str, file_name: &str) -> PathBuf {
    PathBuf::from(sub_path).join(file_name)
}
