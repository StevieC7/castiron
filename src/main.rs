mod file_handling;
mod networking;
mod types;
mod ui;

use iced::{Application, Settings};
use ui::gui::AppLayout;

#[tokio::main]
async fn main() -> iced::Result {
    AppLayout::run(Settings::default())
}
