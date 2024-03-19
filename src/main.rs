mod file_handling;
mod networking;
mod types;

use crate::file_handling::feeds::{add_feed_to_list, get_feed_list};
use crate::networking::downloads::download_episodes;
use crate::networking::feeds::update_feeds;
use crate::types::feeds::FeedMeta;

use std::fs::File;
use std::{fs, io, path::Path};

#[tokio::main]
async fn main() {
    download_episodes().await;
    let open_file: Option<File> = get_feed_list();
    match open_file {
        Some(ref _file) => {
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
        None => println!("Can't update feeds due to unreadable feed list."),
    }
    println!("ADD podcast or LIST shows?");
    let mut mode_selection: String = String::new();
    io::stdin()
        .read_line(&mut mode_selection)
        .expect("Failed to read input.");
    match mode_selection.as_str().trim() {
        "ADD" => {
            println!("You picked ADD");
            match open_file {
                Some(file) => {
                    println!("What feed do you want to follow?");
                    let mut input_url: String = String::new();
                    io::stdin()
                        .read_line(&mut input_url)
                        .expect("Failed to read input.");
                    let feed_result: Option<File> = add_feed_to_list(input_url, file);
                    match feed_result {
                        Some(_file) => {
                            let contents = fs::read_to_string(Path::new("./feed_list.json"))
                                .expect("Oopsie reading saved file");
                            println!("-----Added successfully, contents below-----\n{contents}\n-------------------");
                        }
                        None => println!("Error saving feed to list."),
                    }
                }
                None => println!("Cannot add if feed list does not exist."),
            }
        }
        "LIST" => {
            println!("------Feeds you are following------");
            let read_file: String = fs::read_to_string(Path::new("./feed_list.json"))
                .expect("Oopsie reading saved file.");
            let contents: Result<Vec<FeedMeta>, serde_json::Error> =
                serde_json::from_str(&read_file);
            match contents {
                Ok(values) => {
                    for content in values {
                        println!("{}", content.feed_url)
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
        // TODO: put a function in to read URLS from a file
        _ => println!("You picked wrong."),
    }
}
