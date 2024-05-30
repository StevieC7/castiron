mod file_handling;
mod networking;
mod types;
mod ui;

use crate::networking::{downloads::download_episodes, feeds::update_feeds};
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
                let feed_list_result = update_feeds().await;
                match feed_list_result {
                    Ok(_) => println!("Finished updating feeds."),
                    Err(e) => println!("Error occurred while fetching feed list: {:?}", e),
                }
                let download_result = download_episodes().await;
                match download_result {
                    Ok(_) => println!("Finished downloading episodes."),
                    Err(e) => println!("Error occurred while downloading episodes: {:?}", e),
                }
                Ok(tui_loop().await)
            }
        }
        None => {
            let feed_list_result = update_feeds().await;
            match feed_list_result {
                Ok(_) => println!("Finished updating feeds."),
                Err(e) => println!("Error occurred while fetching feed list: {:?}", e),
            }
            let download_result = download_episodes().await;
            match download_result {
                Ok(_) => println!("Finished downloading episodes."),
                Err(e) => println!("Error occurred while downloading episodes: {:?}", e),
            }
            Ok(tui_loop().await)
        }
    }
}
