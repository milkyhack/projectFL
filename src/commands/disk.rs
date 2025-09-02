use crate::commands::{CommandOutput, SystemCommand};
use crate::utils::run_command;

pub struct DiskUsage;

impl SystemCommand for DiskUsage {
    fn name(&self) -> &str {
        "disk"
    }

    fn run(&self) -> Result<CommandOutput, String> {
        let output = run_command("df", &["-h"])?;
        Ok(CommandOutput::Raw(output))
    }

    fn key(&self) -> String {
        "Disk Usage".to_string()
    }
}
