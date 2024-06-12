mod file_handling;
mod networking;
mod types;
mod ui;

use iced::{Application, Settings};
use ui::gui::Castiron;

#[tokio::main]
async fn main() -> iced::Result {
    Castiron::run(Settings::default())
}
