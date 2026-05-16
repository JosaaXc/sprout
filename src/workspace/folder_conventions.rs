use serde::Serialize;
use std::path::Path;

use crate::context::architecture::{Architecture, ArtifactKind};

#[derive(Debug, Clone, Default, Serialize)]
pub struct FolderConventions {
    pub entity: Option<String>,
    pub repository: Option<String>,
    pub service: Option<String>,
    pub controller: Option<String>,
    pub dto: Option<String>,
    pub mapper: Option<String>,
}

impl FolderConventions {
    pub fn detect(base_path: &Path, feature: &str, arch: Architecture) -> Self {
        Self {
            entity: scan_for_artifact(base_path, feature, arch, ArtifactKind::Entity),
            repository: scan_for_artifact(base_path, feature, arch, ArtifactKind::Repository),
            service: scan_for_artifact(base_path, feature, arch, ArtifactKind::Service),
            controller: scan_for_artifact(base_path, feature, arch, ArtifactKind::Controller),
            dto: scan_for_artifact(base_path, feature, arch, ArtifactKind::Dto),
            mapper: scan_for_artifact(base_path, feature, arch, ArtifactKind::Mapper),
        }
    }

    pub fn folder_for(&self, kind: ArtifactKind) -> Option<&str> {
        match kind {
            ArtifactKind::Entity => self.entity.as_deref(),
            ArtifactKind::Repository => self.repository.as_deref(),
            ArtifactKind::Service | ArtifactKind::ServiceTest => self.service.as_deref(),
            ArtifactKind::Controller | ArtifactKind::ControllerTest => self.controller.as_deref(),
            ArtifactKind::Dto => self.dto.as_deref(),
            ArtifactKind::Mapper => self.mapper.as_deref(),
        }
    }
}

fn candidates_for(kind: ArtifactKind) -> &'static [&'static str] {
    match kind {
        ArtifactKind::Entity => &["entity", "entities", "model", "models", "domain"],
        ArtifactKind::Repository => &[
            "repository",
            "repositories",
            "repo",
            "repos",
            "dao",
            "daos",
        ],
        ArtifactKind::Service | ArtifactKind::ServiceTest => {
            &["service", "services", "usecase", "usecases"]
        }
        ArtifactKind::Controller | ArtifactKind::ControllerTest => {
            &["controller", "controllers", "web", "api", "endpoint", "endpoints"]
        }
        ArtifactKind::Dto => &["dto", "dtos", "payload", "payloads"],
        ArtifactKind::Mapper => &["mapper", "mappers", "converter", "converters"],
    }
}

fn scan_for_artifact(
    base_path: &Path,
    feature: &str,
    arch: Architecture,
    kind: ArtifactKind,
) -> Option<String> {
    let scan_root = scan_root_for(base_path, feature, arch, kind);
    let candidates = candidates_for(kind);
    let default = kind.folder();

    for cand in candidates {
        if *cand == default {
            continue;
        }
        if scan_root.join(cand).is_dir() {
            return Some((*cand).to_string());
        }
    }
    None
}

fn scan_root_for(
    base_path: &Path,
    feature: &str,
    arch: Architecture,
    kind: ArtifactKind,
) -> std::path::PathBuf {
    use Architecture::*;
    use ArtifactKind::*;
    match (arch, kind) {
        (Layered, _) => base_path.to_path_buf(),
        (Modular, _) => base_path.join(feature),
        (Hexagonal, Entity) => base_path.join(feature).join("domain"),
        (Hexagonal, Dto) => base_path.join(feature).join("application"),
        (Hexagonal, Service | ServiceTest) => base_path.join(feature),
        (Hexagonal, Controller | ControllerTest | Repository | Mapper) => {
            base_path.join(feature).join("infrastructure")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn touch_dir(root: &Path, rel: &str) {
        fs::create_dir_all(root.join(rel)).unwrap();
    }

    #[test]
    fn defaults_when_nothing_exists() {
        let tmp = TempDir::new().unwrap();
        let conv = FolderConventions::detect(tmp.path(), "user", Architecture::Modular);
        assert_eq!(conv.entity, None);
        assert_eq!(conv.repository, None);
    }

    #[test]
    fn detects_model_for_entity_modular() {
        let tmp = TempDir::new().unwrap();
        touch_dir(tmp.path(), "user/model");
        let conv = FolderConventions::detect(tmp.path(), "user", Architecture::Modular);
        assert_eq!(conv.entity.as_deref(), Some("model"));
    }

    #[test]
    fn detects_repositories_plural_for_repository_layered() {
        let tmp = TempDir::new().unwrap();
        touch_dir(tmp.path(), "repositories");
        let conv = FolderConventions::detect(tmp.path(), "user", Architecture::Layered);
        assert_eq!(conv.repository.as_deref(), Some("repositories"));
    }

    #[test]
    fn skips_default_folder_so_it_stays_none() {
        let tmp = TempDir::new().unwrap();
        touch_dir(tmp.path(), "user/entity");
        let conv = FolderConventions::detect(tmp.path(), "user", Architecture::Modular);
        // entity is the default — we treat it as "no override needed"
        assert_eq!(conv.entity, None);
    }

    #[test]
    fn first_match_wins_in_priority_order() {
        let tmp = TempDir::new().unwrap();
        // Both "model" and "domain" exist; "model" comes first in candidates.
        touch_dir(tmp.path(), "user/model");
        touch_dir(tmp.path(), "user/domain");
        let conv = FolderConventions::detect(tmp.path(), "user", Architecture::Modular);
        assert_eq!(conv.entity.as_deref(), Some("model"));
    }
}
