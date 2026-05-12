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
    #[arg(long, global = true, help = "Skip generating unit and integration tests")]
    pub skip_test: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(alias = "g", about = "Generate Spring Boot artifacts")]
    Generate(GenerateArgs),
}

#[derive(Debug, Args)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub kind: SchematicKind,
}
