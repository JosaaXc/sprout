pub mod cli;
pub mod commands;
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

use crate::cli::command_tree::{Command, GenerateArgs, SproutCli};
use crate::cli::schematic_kind::SchematicKind;
use crate::context::generation_context::GenerationContext;
use crate::prompts::{DialoguerPrompter, InteractivePrompter};
use crate::rendering::tera_engine::TeraEngine;
use crate::schematics::registry::SchematicRegistry;
use crate::update_check::UpdateProbe;
use crate::workspace::build_tool::{dependency_audit, detect as detect_build_tool};
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
        Command::Generate(args) => generate(args, cli.skip_test),
        Command::List => commands::list::run(),
        Command::Doctor => commands::doctor::run(),
    }
}

fn generate(args: GenerateArgs, skip_test: bool) -> Result<()> {
    let project = ProjectDetector::detect_from_cwd()?;
    println!(
        "{} Detected Spring Boot project at {} (base package: {})",
        style("✓").green().bold(),
        style(project.root().display()).bold(),
        style(project.base_package()).bold()
    );

    let prompter = DialoguerPrompter::new();

    let kind = resolve_kind(args.kind, &prompter)?;
    let raw_name = resolve_name(args.name, kind, &prompter)?;

    let engine = TeraEngine::with_overrides(project.root())?;
    engine.announce_overrides();

    let context = GenerationContext::build(kind, &raw_name, &project, &prompter, skip_test)?;

    if let Ok(build_tool) = detect_build_tool(project.root()) {
        dependency_audit::audit_and_offer_install(build_tool.as_ref(), &context)?;
    }

    let schematic = SchematicRegistry::resolve(kind);
    let outputs = schematic.generate(&context, &engine)?;

    println!();
    let mut writer = DiskFileWriter::new(project.base_path().to_path_buf());
    writer.test_base_path = Some(project.test_base_path().to_path_buf());

    for output in outputs {
        writer.write(output)?;
    }
    println!("\n{} Done.", style("✓").green().bold());
    Ok(())
}

fn resolve_kind(
    from_args: Option<SchematicKind>,
    prompter: &dyn InteractivePrompter,
) -> Result<SchematicKind> {
    match from_args {
        Some(k) => Ok(k),
        None => prompter.ask_schematic_kind(),
    }
}

fn resolve_name(
    from_args: Option<String>,
    kind: SchematicKind,
    prompter: &dyn InteractivePrompter,
) -> Result<String> {
    match from_args {
        Some(n) => Ok(n),
        None => prompter.ask_name(kind),
    }
}
