mod file_handling;
mod networking;
mod types;
mod ui;

use file_handling::setup::{
    create_database_if_not_existing, create_episodes_directory_if_not_existing,
    create_shows_directory_if_not_existing, create_thumbnails_directory_if_not_existing,
};
use iced::{Application, Settings};
use ui::gui::Castiron;

#[tokio::main]
async fn main() -> iced::Result {
    create_shows_directory_if_not_existing().await.unwrap();
    create_episodes_directory_if_not_existing().await.unwrap();
    create_thumbnails_directory_if_not_existing().await.unwrap();
    create_database_if_not_existing().await.unwrap();
    Castiron::run(Settings::default())
}
