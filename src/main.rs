mod file_handling;
mod networking;
mod types;

use crate::file_handling::feeds::{
    add_feed_to_database, add_feed_to_list, get_feed_list, get_feed_list_database,
};
use crate::networking::downloads::download_episodes;
use crate::networking::feeds::update_feeds;
use crate::types::{errors::CustomError, feeds::FeedMeta};

use std::{fs, io, path::Path};

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
    println!("ADD podcast, LIST shows, DB add, DB_LIST");
    let mut mode_selection: String = String::new();
    io::stdin()
        .read_line(&mut mode_selection)
        .expect("Failed to read input.");
    match mode_selection.as_str().trim() {
        "ADD" => {
            println!("You picked ADD");
            match open_file {
                Ok(file) => {
                    println!("What feed do you want to follow?");
                    let mut input_url: String = String::new();
                    io::stdin()
                        .read_line(&mut input_url)
                        .expect("Failed to read input.");
                    let feed_result = add_feed_to_list(input_url, file).await;
                    match feed_result {
                        Ok(_file) => {
                            let contents = fs::read_to_string(Path::new("./feed_list.json"))
                                .expect("Oopsie reading saved file");
                            println!("-----Added successfully, contents below-----\n{contents}\n-------------------");
                        }
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
                }
                Err(_) => println!("Cannot add if feed list does not exist."),
            }
        }
        "LIST" => {
            println!("------Feeds you are following------");
            let read_file = fs::read_to_string(Path::new("./feed_list.json"));
            match read_file {
                Ok(file) => {
                    let contents: Result<Vec<FeedMeta>, serde_json::Error> =
                        serde_json::from_str(&file);
                    match contents {
                        Ok(values) => {
                            for content in values {
                                println!("{}", content.feed_url)
                            }
                        }
                        Err(e) => println!("{e}"),
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
        "DB" => {
            println!("What feed do you want to follow?");
            let mut input_url: String = String::new();
            io::stdin()
                .read_line(&mut input_url)
                .expect("Failed to read input.");
            let result = add_feed_to_database(input_url);
            match result {
                Ok(_) => println!("Added to db."),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        "DB_LIST" => {
            if let Ok(urls) = get_feed_list_database() {
                println!("You did it yay: {:?}", urls)
            } else {
                println!("something wrong")
            }
        }
        // TODO: put a function in to read URLS from an OPML file
        _ => println!("You picked wrong."),
    }
}
