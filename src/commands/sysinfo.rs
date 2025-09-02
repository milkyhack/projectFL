use crate::commands::{CommandOutput, SystemCommand};
use crate::utils::run_command;

pub struct SystemInfo;

impl SystemCommand for SystemInfo {
    fn name(&self) -> &str {
        "sysinfo"
    }

    fn run(&self) -> Result<CommandOutput, String> {
        let raw = if has_lshw() {
            run_command("lshw", &["-short"])?
        } else {
            run_command("uname", &["-a"])?
        };

        let filtered = filter_lshw_output(&raw);
        Ok(CommandOutput::Filtered {
            raw,
            filtered,
        })
    }

    fn key(&self) -> String {
        "System Info".to_string()
    }
}

fn has_lshw() -> bool {
    std::process::Command::new("which")
        .arg("lshw")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn filter_lshw_output(raw: &str) -> String {
    raw.lines()
        .filter(|line| !line.contains("configuration"))
        .collect::<Vec<_>>()
        .join("\n")
}
