pub mod cli;
pub mod context;
pub mod error;
pub mod naming;
pub mod prompts;
pub mod rendering;
pub mod schematics;
pub mod update_check;
pub mod workspace;

use anyhow::Result;
use console::style;

use crate::cli::command_tree::{Command, SproutCli};
use crate::prompts::DialoguerPrompter;
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::registry::SchematicRegistry;
use crate::update_check::UpdateProbe;
use crate::workspace::build_tool::{detect as detect_build_tool, dependency_audit};
use crate::workspace::file_writer::DiskFileWriter;
use crate::workspace::project_detector::ProjectDetector;

pub fn run(cli: SproutCli) -> Result<()> {
    let update_probe = UpdateProbe::start();
    let result = dispatch(cli);
    update_probe.finalize();
    result
}

fn dispatch(cli: SproutCli) -> Result<()> {
    match cli.command {
        Command::Generate(args) => {
            let project = ProjectDetector::detect_from_cwd()?;
            println!(
                "{} Detected Spring Boot project at {} (base package: {})",
                style("✓").green().bold(),
                style(project.root().display()).bold(),
                style(project.base_package()).bold()
            );

            let prompter = DialoguerPrompter::new();
            let engine = TeraEngine::with_embedded_templates()?;
            let writer = DiskFileWriter::new(project.base_path().to_path_buf());

            let raw_name = args.kind.name().to_string();
            let context = args
                .kind
                .collect_context(&raw_name, &project, &prompter)?;

            if let Ok(build_tool) = detect_build_tool(project.root()) {
                dependency_audit::audit_and_offer_install(build_tool.as_ref(), &context)?;
            }

            let schematic = SchematicRegistry::resolve(&args.kind);
            let outputs = schematic.generate(&context, &engine)?;

            println!();
            for output in outputs {
                writer.write(output)?;
            }
            println!("\n{} Done.", style("✓").green().bold());
            Ok(())
        }
    }
}
