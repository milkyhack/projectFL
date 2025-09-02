use std::process::Command;

/// Выполняет системную команду и возвращает stdout
pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Не удалось запустить '{}': {}", cmd, e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
