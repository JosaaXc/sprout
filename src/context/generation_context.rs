use anyhow::Result;
use serde::Serialize;

use crate::cli::schematic_kind::SchematicKind;
use crate::context::architecture::{Architecture, ArtifactKind};
use crate::context::dto_style::DtoStyle;
use crate::context::persistence::Persistence;
use crate::naming::identifier::NameSet;
use crate::prompts::InteractivePrompter;
use crate::workspace::folder_conventions::FolderConventions;
use crate::workspace::project_detector::ProjectContext;

#[derive(Debug, Serialize)]
pub struct PackageMap {
    pub entity: String,
    pub dto: String,
    pub mapper: String,
    pub service: String,
    pub controller: String,
    pub repository: String,
}

#[derive(Debug, Serialize)]
pub struct GenerationContext {
    pub name: NameSet,
    pub base_package: String,
    pub architecture: Architecture,
    pub dto_style: DtoStyle,
    pub persistence: Persistence,
    pub is_jpa: bool,
    pub id_type: String,
    pub repository_base_class: String,
    pub packages: PackageMap,
    pub conventions: FolderConventions,
    pub skip_test: bool,
}

impl GenerationContext {
    pub fn build(
        kind: SchematicKind,
        raw_name: &str,
        project: &ProjectContext,
        prompter: &dyn InteractivePrompter,
        skip_test: bool,
    ) -> Result<Self> {
        let architecture = prompter.ask_architecture()?;

        let dto_style = if kind.needs_dto_style() {
            prompter.ask_dto_style()?
        } else {
            DtoStyle::default()
        };

        let persistence = if kind.needs_persistence() {
            prompter.ask_persistence()?
        } else {
            Persistence::default()
        };

        let name = NameSet::from_raw(raw_name);
        let base_package = project.base_package().to_string();
        let conventions =
            FolderConventions::detect(project.base_path(), &name.kebab, architecture);
        let packages =
            build_package_map(&base_package, &architecture, &name.kebab, &conventions);

        Ok(Self {
            name,
            base_package,
            architecture,
            dto_style,
            persistence,
            is_jpa: persistence.is_jpa(),
            id_type: persistence.id_type().to_string(),
            repository_base_class: persistence.repository_base_class().to_string(),
            packages,
            conventions,
            skip_test,
        })
    }
}

fn build_package_map(
    base_package: &str,
    arch: &Architecture,
    feature: &str,
    conventions: &FolderConventions,
) -> PackageMap {
    let qualify = |artifact: ArtifactKind| -> String {
        let suffix = arch.package_segment_for(feature, artifact, conventions);
        if suffix.is_empty() {
            base_package.to_string()
        } else {
            format!("{base_package}.{suffix}")
        }
    };
    PackageMap {
        entity: qualify(ArtifactKind::Entity),
        dto: qualify(ArtifactKind::Dto),
        mapper: qualify(ArtifactKind::Mapper),
        service: qualify(ArtifactKind::Service),
        controller: qualify(ArtifactKind::Controller),
        repository: qualify(ArtifactKind::Repository),
    }
}
