use anyhow::Result;
use console::style;

use crate::cli::schematic_kind::SchematicKind;
use crate::workspace::build_tool::{detect as detect_build_tool, BuildToolKind};
use crate::workspace::project_detector::{ProjectContext, ProjectDetector};

pub fn run() -> Result<()> {
    let project = ProjectDetector::detect_from_cwd().ok();

    print_project_section(project.as_ref());
    print_schematics_section();
    Ok(())
}

fn print_project_section(project: Option<&ProjectContext>) {
    println!("{}", style("Project").bold().underlined());
    match project {
        Some(p) => {
            println!("  {:<14} {}", "Root", p.root().display());
            println!("  {:<14} {}", "Base package", p.base_package());
            match detect_build_tool(p.root()) {
                Ok(tool) => {
                    let kind = match tool.kind() {
                        BuildToolKind::Maven => "Maven (pom.xml)",
                        BuildToolKind::Gradle => "Gradle (build.gradle)",
                    };
                    println!("  {:<14} {}", "Build tool", kind);
                }
                Err(_) => println!(
                    "  {:<14} {}",
                    "Build tool",
                    style("not detected").dim()
                ),
            }
        }
        None => {
            println!(
                "  {}",
                style("not a Spring Boot project (no src/main/java found)").yellow()
            );
        }
    }
    println!();
}

fn print_schematics_section() {
    println!("{}", style("Schematics").bold().underlined());
    for kind in SchematicKind::all() {
        let short = match kind {
            SchematicKind::Resource => "resource",
            SchematicKind::Service => "service",
            SchematicKind::Controller => "controller",
            SchematicKind::Entity => "entity",
            SchematicKind::Repository => "repository",
            SchematicKind::Dto => "dto",
            SchematicKind::Mapper => "mapper",
        };
        println!(
            "  {} {:<11} {}",
            style("●").green(),
            style(short).bold(),
            style(kind.label()).dim()
        );
    }
    println!();
    println!(
        "  {} sprout g <kind> <name>   (or just `sprout g` for interactive)",
        style("Usage:").dim()
    );
    println!();
}
