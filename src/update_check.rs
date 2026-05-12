use console::{measure_text_width, style};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use update_informer::{registry, Check};

const REPO: &str = "JosaaXc/sprout";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24);
const FINALIZE_TIMEOUT: Duration = Duration::from_millis(500);

const INSTALL_CMD_UNIX: &str =
    "curl -fsSL https://raw.githubusercontent.com/JosaaXc/sprout/main/install.sh | sh";
const INSTALL_CMD_WINDOWS: &str =
    "irm https://raw.githubusercontent.com/JosaaXc/sprout/main/install.ps1 | iex";

pub struct UpdateProbe {
    receiver: Option<mpsc::Receiver<Option<String>>>,
}

impl UpdateProbe {
    pub fn start() -> Self {
        if is_disabled() {
            return Self { receiver: None };
        }

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let informer = update_informer::new(registry::GitHub, REPO, CURRENT_VERSION)
                .interval(CHECK_INTERVAL);
            let latest = informer
                .check_version()
                .ok()
                .flatten()
                .map(|v| v.to_string());
            let _ = tx.send(latest);
        });

        Self { receiver: Some(rx) }
    }

    pub fn finalize(self) {
        let Some(rx) = self.receiver else {
            return;
        };
        if let Ok(Some(new_version)) = rx.recv_timeout(FINALIZE_TIMEOUT) {
            if new_version != CURRENT_VERSION {
                print_update_box(&new_version);
            }
        }
    }
}

fn is_disabled() -> bool {
    std::env::var_os("SPROUT_DISABLE_UPDATE_CHECK").is_some()
        || std::env::var_os("NO_UPDATE_CHECK").is_some()
}

fn install_command() -> &'static str {
    if cfg!(target_os = "windows") {
        INSTALL_CMD_WINDOWS
    } else {
        INSTALL_CMD_UNIX
    }
}

fn print_update_box(new_version: &str) {
    let header = format!(
        "📦  A new version of Sprout is available: {} → {}",
        style(format!("v{CURRENT_VERSION}")).dim(),
        style(format!("v{}", strip_v_prefix(new_version))).cyan().bold(),
    );
    let body = format!(
        "{} {}",
        style("Run:").dim(),
        style(install_command()).yellow()
    );
    let lines = [header, body];

    let widths: Vec<usize> = lines.iter().map(|l| measure_text_width(l)).collect();
    let max_w = *widths.iter().max().expect("at least one line");
    let pad = 2usize;
    let inner = max_w + pad * 2;

    let border_run = "─".repeat(inner);
    let top = format!(
        "{}{}{}",
        style("╭").cyan(),
        style(&border_run).cyan(),
        style("╮").cyan()
    );
    let bot = format!(
        "{}{}{}",
        style("╰").cyan(),
        style(&border_run).cyan(),
        style("╯").cyan()
    );

    eprintln!();
    eprintln!("{top}");
    for (line, width) in lines.iter().zip(widths.iter()) {
        let trailing = max_w - width + pad;
        eprintln!(
            "{}{}{}{}{}",
            style("│").cyan(),
            " ".repeat(pad),
            line,
            " ".repeat(trailing),
            style("│").cyan(),
        );
    }
    eprintln!("{bot}");
}

fn strip_v_prefix(v: &str) -> &str {
    v.strip_prefix('v').unwrap_or(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_command_matches_target_os() {
        let cmd = install_command();
        if cfg!(target_os = "windows") {
            assert!(cmd.contains("install.ps1"));
        } else {
            assert!(cmd.contains("install.sh"));
        }
    }

    #[test]
    fn disabling_via_env_var_short_circuits() {
        std::env::set_var("SPROUT_DISABLE_UPDATE_CHECK", "1");
        let probe = UpdateProbe::start();
        assert!(probe.receiver.is_none());
        std::env::remove_var("SPROUT_DISABLE_UPDATE_CHECK");
    }
}
