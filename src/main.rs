use anyhow::Result;
use clap::Parser;

use sprout::cli::command_tree::SproutCli;

fn main() -> Result<()> {
    let cli = SproutCli::parse();
    sprout::run(cli)
}
