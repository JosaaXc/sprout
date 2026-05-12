use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::context::generation_context::GenerationContext;
use crate::workspace::build_tool::{
    required_dependencies_for, BuildTool, BuildToolKind, DependencyCoord,
};

pub struct AuditReport {
    pub missing: Vec<DependencyCoord>,
}

impl AuditReport {
    pub fn is_clean(&self) -> bool {
        self.missing.is_empty()
    }
}

pub fn audit(build_tool: &dyn BuildTool, ctx: &GenerationContext) -> Result<AuditReport> {
    let mut missing = Vec::new();
    for dep in required_dependencies_for(ctx) {
        if !build_tool.has_dependency(dep.artifact_id)? {
            missing.push(dep);
        }
    }
    Ok(AuditReport { missing })
}

pub fn audit_and_offer_install(
    build_tool: &dyn BuildTool,
    ctx: &GenerationContext,
) -> Result<()> {
    let report = audit(build_tool, ctx)?;
    if report.is_clean() {
        return Ok(());
    }

    print_warning(build_tool, &report);

    let prompt = format!(
        "Would you like Sprout to add {} now?",
        if report.missing.len() == 1 {
            "the missing dependency"
        } else {
            "the missing dependencies"
        }
    );
    let answer = if std::env::var("SPROUT_NON_INTERACTIVE").is_ok() {
        true
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(true)
            .show_default(true)
            .interact()
            .unwrap_or(false)
    };

    if !answer {
        println!(
            "{} Generated files may not compile until the dependencies above are present.",
            style("ℹ").blue().bold()
        );
        return Ok(());
    }

    for dep in &report.missing {
        build_tool.install(dep)?;
        println!(
            "  {} {} ({}:{}{})",
            style("INSTALL").green().bold(),
            dep.artifact_id,
            dep.group_id,
            dep.artifact_id,
            dep.version.map(|v| format!(":{v}")).unwrap_or_default()
        );
    }
    println!(
        "  {} Re-run your build to refresh the dependency graph.",
        style("✓").green().bold()
    );
    Ok(())
}

fn print_warning(build_tool: &dyn BuildTool, report: &AuditReport) {
    let tool_label = match build_tool.kind() {
        BuildToolKind::Maven => "Maven",
        BuildToolKind::Gradle => "Gradle",
    };
    println!();
    println!(
        "{} {} {}",
        style("⚠️").yellow(),
        style("Missing dependencies in").yellow().bold(),
        style(build_tool.manifest_path().display()).bold()
    );
    println!(
        "   Sprout's templates rely on these and {} is the active build tool:",
        tool_label
    );
    for dep in &report.missing {
        println!(
            "     {} {}:{}{}  {}",
            style("•").yellow(),
            dep.group_id,
            dep.artifact_id,
            dep.version.map(|v| format!(":{v}")).unwrap_or_default(),
            style(format!("— {}", dep.purpose)).dim()
        );
    }
    println!();
}
