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

// use networking::feeds::update_feeds;

// use crate::networking::downloads::download_episodes;
// use crate::ui::tui::tui_loop;

// #[tokio::main]
// async fn main() {
//     let feed_list_result = update_feeds().await;
//     // call update_feeds here
//     match feed_list_result {
//         Ok(_) => println!("Finished updating feeds."),
//         Err(e) => println!("Error occurred while fetching feed list: {:?}", e),
//     }
//     let download_result = download_episodes().await;
//     match download_result {
//         Ok(_) => println!("Finished downloading episodes."),
//         Err(e) => println!("Error occurred while downloading episodes: {:?}", e),
//     }
//     tui_loop().await;
// }
