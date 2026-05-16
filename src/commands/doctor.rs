use anyhow::Result;
use console::style;

use crate::workspace::build_tool::{
    detect as detect_build_tool, required_dependencies_for_kinds, BuildToolKind, DepKind,
    DependencyCoord,
};
use crate::workspace::project_detector::ProjectDetector;

pub fn run() -> Result<()> {
    let mut critical_failures = 0u32;
    let mut warnings = 0u32;

    let project = match ProjectDetector::detect_from_cwd() {
        Ok(p) => {
            println!("{} Spring Boot project detected", style("✓").green().bold());
            println!("    {:<14} {}", "Root", p.root().display());
            println!("    {:<14} {}", "Base package", p.base_package());
            println!("    {:<14} {}", "Source root", p.source_root().display());
            println!();
            Some(p)
        }
        Err(e) => {
            critical_failures += 1;
            println!("{} Spring Boot project not detected", style("✗").red().bold());
            println!("    {}", style(format!("{e:#}")).red());
            println!();
            None
        }
    };

    let build_tool = match project.as_ref().and_then(|p| detect_build_tool(p.root()).ok()) {
        Some(tool) => {
            let label = match tool.kind() {
                BuildToolKind::Maven => "Maven (pom.xml)",
                BuildToolKind::Gradle => "Gradle (build.gradle)",
            };
            println!("{} Build tool: {}", style("✓").green().bold(), label);
            println!();
            Some(tool)
        }
        None => {
            if project.is_some() {
                warnings += 1;
                println!(
                    "{} No build manifest detected (pom.xml or build.gradle)",
                    style("!").yellow().bold()
                );
                println!();
            }
            None
        }
    };

    if let Some(tool) = build_tool.as_ref() {
        println!("{}", style("Dependencies").bold().underlined());
        // Audit every dep Sprout could possibly need so the user gets a complete picture.
        let all_deps =
            required_dependencies_for_kinds(&[DepKind::Common, DepKind::Jpa, DepKind::Mongo]);
        for dep in &all_deps {
            match tool.has_dependency(dep.artifact_id) {
                Ok(true) => print_dep_present(dep),
                Ok(false) => {
                    warnings += 1;
                    print_dep_missing(dep);
                }
                Err(e) => {
                    warnings += 1;
                    println!(
                        "    {} {} {}",
                        style("?").yellow().bold(),
                        dep.artifact_id,
                        style(format!("(could not check: {e})")).dim()
                    );
                }
            }
        }
        println!();
    }

    print_summary(critical_failures, warnings);

    if critical_failures > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn print_dep_present(dep: &DependencyCoord) {
    let version = dep
        .version
        .map(|v| format!("{v}"))
        .unwrap_or_else(|| "managed".into());
    println!(
        "    {} {:<48} {}",
        style("✓").green().bold(),
        dep.artifact_id,
        style(version).dim()
    );
}

fn print_dep_missing(dep: &DependencyCoord) {
    println!(
        "    {} {:<48} {}",
        style("✗").red().bold(),
        dep.artifact_id,
        style("missing").yellow()
    );
}

fn print_summary(critical_failures: u32, warnings: u32) {
    println!("{}", style("Summary").bold().underlined());
    if critical_failures == 0 && warnings == 0 {
        println!(
            "  {} Project is ready for Sprout. Run `sprout g` to generate.",
            style("✓").green().bold()
        );
    } else if critical_failures == 0 {
        println!(
            "  {} {warnings} warning(s). Run `sprout g resource <name>` and accept the dep installer to fix missing dependencies.",
            style("!").yellow().bold()
        );
    } else {
        println!(
            "  {} {critical_failures} critical issue(s). Sprout cannot operate until they are fixed.",
            style("✗").red().bold()
        );
    }
}
