use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::workspace::build_tool::{BuildTool, BuildToolKind, DependencyCoord};

pub struct MavenBuildTool {
    manifest: PathBuf,
}

impl MavenBuildTool {
    pub fn new(manifest: PathBuf) -> Self {
        Self { manifest }
    }
}

impl BuildTool for MavenBuildTool {
    fn kind(&self) -> BuildToolKind {
        BuildToolKind::Maven
    }

    fn manifest_path(&self) -> &Path {
        &self.manifest
    }

    fn has_dependency(&self, artifact_id: &str) -> Result<bool> {
        let contents = fs::read_to_string(&self.manifest)
            .with_context(|| format!("reading {}", self.manifest.display()))?;
        Ok(contains_artifact(&contents, artifact_id))
    }

    fn install(&self, dep: &DependencyCoord) -> Result<()> {
        let contents = fs::read_to_string(&self.manifest)
            .with_context(|| format!("reading {}", self.manifest.display()))?;
        let snippet = build_snippet(dep);
        let updated = inject_into_dependencies(&contents, &snippet)?;
        fs::write(&self.manifest, updated)
            .with_context(|| format!("writing {}", self.manifest.display()))?;
        Ok(())
    }
}

fn contains_artifact(pom: &str, artifact_id: &str) -> bool {
    static ARTIFACT_TAG: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"<artifactId>\s*([^<\s]+)\s*</artifactId>").unwrap());
    ARTIFACT_TAG
        .captures_iter(pom)
        .any(|c| c.get(1).map(|m| m.as_str()) == Some(artifact_id))
}

fn build_snippet(dep: &DependencyCoord) -> String {
    let version_line = dep
        .version
        .map(|v| format!("            <version>{v}</version>\n"))
        .unwrap_or_default();
    format!(
        "        <dependency>\n            <groupId>{}</groupId>\n            <artifactId>{}</artifactId>\n{}        </dependency>\n",
        dep.group_id, dep.artifact_id, version_line
    )
}

fn inject_into_dependencies(pom: &str, snippet: &str) -> Result<String> {
    let idx = locate_main_dependencies_close(pom).ok_or_else(|| {
        anyhow!("could not locate the project-level </dependencies> in pom.xml; add the snippet manually")
    })?;
    let mut updated = String::with_capacity(pom.len() + snippet.len());
    updated.push_str(&pom[..idx]);
    updated.push_str(snippet);
    updated.push_str(&pom[idx..]);
    Ok(updated)
}

fn locate_main_dependencies_close(pom: &str) -> Option<usize> {
    let search_start = pom
        .find("<dependencyManagement>")
        .and_then(|s| {
            pom[s..]
                .find("</dependencyManagement>")
                .map(|e| s + e + "</dependencyManagement>".len())
        })
        .unwrap_or(0);
    pom[search_start..]
        .find("</dependencies>")
        .map(|n| search_start + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const POM: &str = r#"<project>
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
    </dependencies>
</project>
"#;

    #[test]
    fn detects_present_artifact() {
        assert!(contains_artifact(POM, "spring-boot-starter-web"));
    }

    #[test]
    fn detects_missing_artifact() {
        assert!(!contains_artifact(POM, "mapstruct"));
    }

    #[test]
    fn injects_dependency_before_closing_tag() {
        let dep = DependencyCoord {
            group_id: "org.mapstruct",
            artifact_id: "mapstruct",
            version: Some("1.5.5.Final"),
            purpose: "",
        };
        let injected = inject_into_dependencies(POM, &build_snippet(&dep)).unwrap();
        assert!(injected.contains("<artifactId>mapstruct</artifactId>"));
        assert!(injected.contains("<version>1.5.5.Final</version>"));
        let close_idx = injected.find("</dependencies>").unwrap();
        let mapstruct_idx = injected.find("mapstruct").unwrap();
        assert!(mapstruct_idx < close_idx);
    }

    #[test]
    fn skips_dependency_management_block() {
        let pom = r#"<project>
    <dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-dependencies</artifactId>
                <version>3.3.0</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
        </dependencies>
    </dependencyManagement>
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
    </dependencies>
</project>
"#;
        let dep = DependencyCoord {
            group_id: "org.mapstruct",
            artifact_id: "mapstruct",
            version: Some("1.5.5.Final"),
            purpose: "",
        };
        let injected = inject_into_dependencies(pom, &build_snippet(&dep)).unwrap();
        let mapstruct_idx = injected.find("<artifactId>mapstruct</artifactId>").unwrap();
        let mgmt_close = injected.find("</dependencyManagement>").unwrap();
        assert!(
            mapstruct_idx > mgmt_close,
            "mapstruct must land in the project-level <dependencies>, not in dependencyManagement"
        );
    }
}
