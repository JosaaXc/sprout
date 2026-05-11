use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::workspace::build_tool::{BuildTool, BuildToolKind, DependencyCoord};

pub struct GradleBuildTool {
    manifest: PathBuf,
}

impl GradleBuildTool {
    pub fn new(manifest: PathBuf) -> Self {
        Self { manifest }
    }
}

impl BuildTool for GradleBuildTool {
    fn kind(&self) -> BuildToolKind {
        BuildToolKind::Gradle
    }

    fn manifest_path(&self) -> &Path {
        &self.manifest
    }

    fn has_dependency(&self, artifact_id: &str) -> Result<bool> {
        let contents = fs::read_to_string(&self.manifest)
            .with_context(|| format!("reading {}", self.manifest.display()))?;
        Ok(contents.contains(&format!(":{artifact_id}:")) || contents.contains(&format!(":{artifact_id}\"")))
    }

    fn install(&self, dep: &DependencyCoord) -> Result<()> {
        let mut contents = fs::read_to_string(&self.manifest)
            .with_context(|| format!("reading {}", self.manifest.display()))?;
        let coord = match dep.version {
            Some(v) => format!("{}:{}:{}", dep.group_id, dep.artifact_id, v),
            None => format!("{}:{}", dep.group_id, dep.artifact_id),
        };
        let line = format!("    implementation(\"{coord}\")\n");

        if let Some(idx) = contents.find("dependencies {") {
            let insert_at = contents[idx..]
                .find('\n')
                .map(|n| idx + n + 1)
                .unwrap_or(contents.len());
            contents.insert_str(insert_at, &line);
        } else {
            contents.push_str("\ndependencies {\n");
            contents.push_str(&line);
            contents.push_str("}\n");
        }
        fs::write(&self.manifest, contents)
            .with_context(|| format!("writing {}", self.manifest.display()))?;
        Ok(())
    }
}
