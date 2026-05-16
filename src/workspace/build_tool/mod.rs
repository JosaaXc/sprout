use anyhow::{anyhow, Result};
use std::path::Path;

use crate::context::generation_context::GenerationContext;

pub mod dependency_audit;
pub mod gradle;
pub mod maven;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildToolKind {
    Maven,
    Gradle,
}

#[derive(Debug, Clone, Copy)]
pub struct DependencyCoord {
    pub group_id: &'static str,
    pub artifact_id: &'static str,
    pub version: Option<&'static str>,
    pub purpose: &'static str,
}

pub trait BuildTool {
    fn kind(&self) -> BuildToolKind;
    fn manifest_path(&self) -> &Path;
    fn has_dependency(&self, artifact_id: &str) -> Result<bool>;
    fn install(&self, dep: &DependencyCoord) -> Result<()>;
}

pub fn detect(project_root: &Path) -> Result<Box<dyn BuildTool>> {
    let pom = project_root.join("pom.xml");
    if pom.exists() {
        return Ok(Box::new(maven::MavenBuildTool::new(pom)));
    }
    let gradle = project_root.join("build.gradle");
    let gradle_kts = project_root.join("build.gradle.kts");
    if gradle.exists() {
        return Ok(Box::new(gradle::GradleBuildTool::new(gradle)));
    }
    if gradle_kts.exists() {
        return Ok(Box::new(gradle::GradleBuildTool::new(gradle_kts)));
    }
    Err(anyhow!(
        "no build manifest found (pom.xml or build.gradle[.kts]) at {}",
        project_root.display()
    ))
}

const MAPSTRUCT: DependencyCoord = DependencyCoord {
    group_id: "org.mapstruct",
    artifact_id: "mapstruct",
    version: Some("1.5.5.Final"),
    purpose: "DTO ↔ entity mapping used by generated @Mapper interfaces",
};

const VALIDATION: DependencyCoord = DependencyCoord {
    group_id: "org.springframework.boot",
    artifact_id: "spring-boot-starter-validation",
    version: None,
    purpose: "Bean Validation (@NotBlank, @Valid) used by generated DTOs and controllers",
};

const DATA_JPA: DependencyCoord = DependencyCoord {
    group_id: "org.springframework.boot",
    artifact_id: "spring-boot-starter-data-jpa",
    version: None,
    purpose: "JPA / Hibernate runtime required by JpaRepository<Entity, Long>",
};

const DATA_MONGO: DependencyCoord = DependencyCoord {
    group_id: "org.springframework.boot",
    artifact_id: "spring-boot-starter-data-mongodb",
    version: None,
    purpose: "Spring Data MongoDB runtime required by MongoRepository<Entity, String>",
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepKind {
    Common,
    Jpa,
    Mongo,
}

pub fn dependencies_for_kinds(kinds: &[DepKind]) -> Vec<DependencyCoord> {
    let mut deps = Vec::new();
    for kind in kinds {
        match kind {
            DepKind::Common => {
                deps.push(MAPSTRUCT);
                deps.push(VALIDATION);
            }
            DepKind::Jpa => deps.push(DATA_JPA),
            DepKind::Mongo => deps.push(DATA_MONGO),
        }
    }
    deps
}

pub fn required_dependencies_for(ctx: &GenerationContext) -> Vec<DependencyCoord> {
    let kinds = if ctx.is_jpa {
        [DepKind::Common, DepKind::Jpa]
    } else {
        [DepKind::Common, DepKind::Mongo]
    };
    dependencies_for_kinds(&kinds)
}

pub fn required_dependencies_for_kinds(kinds: &[DepKind]) -> Vec<DependencyCoord> {
    dependencies_for_kinds(kinds)
}
