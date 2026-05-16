use clap::{Args, Parser, Subcommand};

use crate::cli::schematic_kind::SchematicKind;

#[derive(Debug, Parser)]
#[command(
    name = "sprout",
    version,
    about = "Ultra-fast Spring Boot scaffolding CLI",
    long_about = "Sprout is a Rust-based CLI that generates Spring Boot code (entities, DTOs, services, controllers, mappers) from a small set of opinionated, configurable templates."
)]
pub struct SproutCli {
    #[arg(
        long,
        global = true,
        help = "Skip generating unit and integration tests"
    )]
    pub skip_test: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(
        alias = "g",
        about = "Generate Spring Boot artifacts (interactive when args are omitted)"
    )]
    Generate(GenerateArgs),

    #[command(
        alias = "ls",
        about = "List available schematics and detected project context"
    )]
    List,

    #[command(about = "Diagnose the current project for Sprout compatibility")]
    Doctor,
}

#[derive(Debug, Args)]
pub struct GenerateArgs {
    #[arg(value_enum, help = "What to generate (omit for interactive picker)")]
    pub kind: Option<SchematicKind>,

    #[arg(
        help = "Resource name (omit for interactive prompt). Accepts kebab-case or PascalCase, e.g. user or user-account"
    )]
    pub name: Option<String>,
}
