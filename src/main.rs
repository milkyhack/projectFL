mod commands;
mod localization;
mod ui;
mod utils;

use crate::localization::Localization;
use commands::{disk::DiskUsage, installed::InstalledPrograms, sysinfo::SystemInfo, SystemCommand};
use cursive::views::TextView;
use cursive::CursiveRunnable;
use chrono::Local;
use std::sync::Arc;
use tokio::{sync::Mutex, time::Duration};

fn build_commands() -> Vec<Box<dyn SystemCommand + Send + Sync>> {
    vec![
        Box::new(InstalledPrograms),
        Box::new(SystemInfo),
        Box::new(DiskUsage),
    ]
}

fn detect_system_language() -> String {
    std::env::var("LANG")
        .unwrap_or_else(|_| "en".to_string())
        .split('.')
        .next()
        .unwrap_or("en")
        .split('_')
        .next()
        .unwrap_or("en")
        .to_lowercase()
}

#[tokio::main]
async fn main() {
    let mut siv = cursive::default();

    let mut loc = Localization::new();
    let lang = detect_system_language();
    loc.set_language(&lang);

    let localization = Arc::new(Mutex::new(loc));
    let commands = build_commands();

    ui::update_ui(&mut siv, localization.clone(), commands).await;

    let sink = siv.cb_sink().clone();
    tokio::spawn(async move {
        loop {
            let time = Local::now().format("%H:%M:%S").to_string();
            let _ = sink.send(Box::new(move |s| {
                if let Some(mut view) = s.find_name::<TextView>("clock_view") {
                    view.set_content(time.clone());
                }
            }));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    siv.run();
}
