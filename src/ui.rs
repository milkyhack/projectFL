use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, DummyView, LinearLayout, TextView},
    Cursive,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Local;

use crate::localization::Localization;
use crate::commands::{SystemCommand, CommandOutput};

pub async fn update_ui(
    siv: &mut Cursive,
    localization: Arc<Mutex<Localization>>,
    commands: Vec<Box<dyn SystemCommand + Send + Sync>>,
) {
    siv.pop_layer();

    let loc = localization.lock().await;

    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H:%M:%S").to_string();

    let status_bar = LinearLayout::horizontal()
        .child(TextView::new(date))
        .child(DummyView.full_width())
        .child(TextView::new(time).h_align(HAlign::Right).with_name("clock_view"));

    let greeting = TextView::new(loc.translate("Welcome to the OSDT diagnostic system").await)
        .h_align(HAlign::Center);

    let mut button_column = LinearLayout::vertical();

    for cmd in commands {
        let label = loc.translate(&cmd.key()).await;
        let label_clone = label.clone();
        let loc_clone = localization.clone();
        let cmd_arc = Arc::new(cmd); // оборачиваем команду в Arc

        button_column.add_child(
            Button::new(label_clone, move |s| {
                let loc = loc_clone.clone();
                let cb_sink = s.cb_sink().clone();
                let cmd_clone = cmd_arc.clone(); // клонируем Arc

                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        let loc = loc.lock().await;
                        let result = cmd_clone.run();
                        let translated_title = loc.translate(&cmd_clone.key()).await;

                        let _ = cb_sink.send(Box::new(move |siv| {
                            match result {
                                Ok(CommandOutput::Raw(raw)) => {
                                    siv.add_layer(
                                        Dialog::new()
                                            .title(translated_title)
                                            .content(TextView::new(raw).scrollable())
                                            .button("OK", |s| {
                                                s.pop_layer();
                                            }),
                                    );
                                }
                                Ok(CommandOutput::Filtered { raw, filtered }) => {
                                    show_filtered_toggle(siv, translated_title, raw, filtered);
                                }
                                Err(e) => {
                                    siv.add_layer(
                                        Dialog::new()
                                            .title("Error")
                                            .content(TextView::new(e))
                                            .button("OK", |s| {
                                                s.pop_layer();
                                            }),
                                    );
                                }
                            }
                        }));
                    });
                });
            })
            .full_width(),
        );
    }

    button_column.add_child(
        Button::new(loc.translate("Exit").await, |s| s.quit())
            .full_width(),
    );

    let footer = TextView::new(
        loc.translate("Created for Linux diagnostics | Language is selected automatically")
            .await,
    )
    .h_align(HAlign::Center);

    let layout = LinearLayout::vertical()
        .child(status_bar)
        .child(DummyView)
        .child(greeting)
        .child(DummyView)
        .child(button_column)
        .child(DummyView)
        .child(footer);

    let dialog = Dialog::around(layout).title("OSDT");
    siv.add_layer(dialog);
}

fn show_filtered_toggle(s: &mut Cursive, title: String, raw: String, filtered: String) {
    let raw_clone = raw.clone();
    s.add_layer(
        Dialog::new()
            .title(format!("{} (filtered)", title))
            .content(TextView::new(filtered).scrollable())
            .button("Полный вывод", move |s| {
                s.pop_layer();
                s.add_layer(
                    Dialog::new()
                        .title(title.clone())
                        .content(TextView::new(raw_clone.clone()).scrollable())
                        .button("OK", |s| {
                            s.pop_layer();
                        }),
                );
            })
            .button("OK", |s| {
                s.pop_layer();
            }),
    );
}
