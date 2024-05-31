mod file_handling;
mod networking;
mod types;
mod ui;

use crate::ui::tui::tui_loop;
use iced::{Application, Settings};
use std::env::args;
use ui::gui::AppLayout;

#[tokio::main]
async fn main() -> iced::Result {
    let mode = args().nth(1);
    match mode {
        Some(flag) => {
            if flag == String::from("--gui") {
                AppLayout::run(Settings::default())
            } else {
                Ok(tui_loop().await)
            }
        }
        None => Ok(tui_loop().await),
    }
}
