use crate::commands::{CommandOutput, SystemCommand};
use crate::utils::run_command;

pub struct InstalledPrograms;

impl SystemCommand for InstalledPrograms {
    fn name(&self) -> &str {
        "installed"
    }

    fn run(&self) -> Result<CommandOutput, String> {
        let output = run_command("dpkg-query", &["-l"])?;
        Ok(CommandOutput::Raw(output))
    }

    fn key(&self) -> String {
        "Installed Programs".to_string()
    }
}
