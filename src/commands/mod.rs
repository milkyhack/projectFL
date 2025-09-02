pub mod disk;
pub mod installed;
pub mod sysinfo;

use crate::commands::{disk::DiskUsage, installed::InstalledPrograms, sysinfo::SystemInfo};

/// Команда возвращает текст (сырой или отфильтрованный)
pub enum CommandOutput {
    Raw(String),
    Filtered {
        raw: String,
        filtered: String,
    },
}

/// Базовый интерфейс для команд
pub trait SystemCommand {
    fn name(&self) -> &str; // название команды, опционально
    fn run(&self) -> Result<CommandOutput, String>;
    fn key(&self) -> String; // используется для перевода
}
