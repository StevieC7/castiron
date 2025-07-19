mod file_handling;
mod networking;
mod types;
mod ui;

use iced::application;
use ui::gui::Castiron;

fn main() -> iced::Result {
    // TODO: move into the application itself as tasks spawned at startup
    // create_shows_directory_if_not_existing().await.unwrap();
    // create_episodes_directory_if_not_existing().await.unwrap();
    // create_thumbnails_directory_if_not_existing().await.unwrap();
    // create_database_if_not_existing().await.unwrap();

    // TODO: fix styling / theming
    // TODO: fix messages / tasks
    application("Castiron", Castiron::update, Castiron::view)
        .subscription(Castiron::subscription)
        .run()
}
