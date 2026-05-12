use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

static PACKAGE_DECLARATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*package\s+([a-zA-Z_][\w.]*)\s*;").unwrap());

const SPRING_BOOT_APPLICATION_MARKER: &str = "@SpringBootApplication";

#[derive(Debug, Clone)]
pub struct ProjectContext {
    root: PathBuf,
    base_path: PathBuf,
    base_package: String,
    application_class: PathBuf,
}

impl ProjectContext {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    pub fn base_package(&self) -> &str {
        &self.base_package
    }

    pub fn application_class(&self) -> &Path {
        &self.application_class
    }

    pub fn source_root(&self) -> PathBuf {
        self.root.join("src/main/java")
    }

    pub fn test_base_path(&self) -> PathBuf {
        let package_path = self.base_package.replace('.', "/");
        self.root.join("src/test/java").join(package_path)
    }
}

pub struct ProjectDetector;

impl ProjectDetector {
    pub fn detect_from_cwd() -> Result<ProjectContext> {
        let cwd = std::env::current_dir()?;
        Self::detect_from(&cwd)
    }

    pub fn detect_from(root: &Path) -> Result<ProjectContext> {
        let source_root = root.join("src/main/java");
        if !source_root.exists() {
            return Err(anyhow!(
                "sprout must be run from a Spring Boot project root (src/main/java not found at {})",
                root.display()
            ));
        }

        let (application_class, base_package) = find_spring_boot_application(&source_root)
            .with_context(|| {
                format!(
                    "could not find a @SpringBootApplication class under {}",
                    source_root.display()
                )
            })?;

        let base_path = application_class
            .parent()
            .ok_or_else(|| anyhow!("application class has no parent directory"))?
            .to_path_buf();

        Ok(ProjectContext {
            root: root.to_path_buf(),
            base_path,
            base_package,
            application_class,
        })
    }
}

fn find_spring_boot_application(source_root: &Path) -> Result<(PathBuf, String)> {
    for entry in walkdir::WalkDir::new(source_root).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().map_or(true, |ext| ext != "java") {
            continue;
        }
        let contents = match fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if !contents.contains(SPRING_BOOT_APPLICATION_MARKER) {
            continue;
        }
        let pkg = PACKAGE_DECLARATION
            .captures(&contents)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                anyhow!(
                    "found @SpringBootApplication at {} but no package declaration could be parsed",
                    entry.path().display()
                )
            })?;
        return Ok((entry.path().to_path_buf(), pkg));
    }
    Err(anyhow!("no @SpringBootApplication class found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write(root: &Path, rel: &str, contents: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn detects_base_package_from_application_class() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "src/main/java/com/acme/store/StoreApplication.java",
            "package com.acme.store;\n\nimport org.springframework.boot.autoconfigure.SpringBootApplication;\n\n@SpringBootApplication\npublic class StoreApplication {}\n",
        );
        write(
            tmp.path(),
            "src/main/java/com/acme/store/other/Util.java",
            "package com.acme.store.other; class Util {}\n",
        );

        let project = ProjectDetector::detect_from(tmp.path()).unwrap();
        assert_eq!(project.base_package(), "com.acme.store");
        assert!(project.base_path().ends_with("com/acme/store"));
    }

    #[test]
    fn fails_when_no_application_class_found() {
        let tmp = TempDir::new().unwrap();
        write(
            tmp.path(),
            "src/main/java/com/acme/Foo.java",
            "package com.acme; class Foo {}\n",
        );
        assert!(ProjectDetector::detect_from(tmp.path()).is_err());
    }
}
