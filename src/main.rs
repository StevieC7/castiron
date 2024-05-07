mod file_handling;
mod networking;
mod types;
mod ui;

use crate::file_handling::feeds::get_feed_list;
use crate::networking::downloads::download_episodes;
use crate::networking::feeds::update_feeds;
use crate::types::{errors::CustomError, feeds::FeedMeta};
use crate::ui::tui::tui_loop;

use std::{fs, path::Path};

#[tokio::main]
async fn main() {
    let download_result = download_episodes().await;
    match download_result {
        Ok(_) => println!("Finished downloading episodes."),
        Err(e) => match e {
            CustomError::IOError(_) => {
                println!("IO Error while adding feed to list")
            }
            CustomError::ReqwestError(_) => {
                println!("Network request error while adding feed to list.")
            }
            CustomError::SerdeJsonError(_) => {
                println!("Serialization error while adding feed to list.")
            }
            CustomError::XmlError(_) => {
                println!("XML Error while adding feed to list")
            }
            CustomError::SqlError(_) => {
                println!("SQL Error while adding feed to list")
            }
        },
    }
    let open_file = get_feed_list();
    match open_file {
        Ok(_) => {
            let read_file: String = fs::read_to_string(Path::new("./feed_list.json"))
                .expect("Oopsie reading saved file.");
            match read_file.len() {
                0 => println!("Found no feeds to update."),
                _ => {
                    let subscribed_feeds: Result<Vec<FeedMeta>, serde_json::Error> =
                        serde_json::from_str(&read_file);
                    match subscribed_feeds {
                        Ok(feeds) => update_feeds(feeds).await,
                        Err(e) => println!("Error reading feed list: {e}"),
                    }
                }
            }
        }
        Err(_) => println!("Can't update feeds due to unreadable feed list."),
    }
    tui_loop(open_file).await;
}
