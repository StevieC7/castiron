use std::{
    fs::{read_to_string, File},
    io::{self, Error},
    path::Path,
};

use crate::file_handling::feeds::{add_feed_to_database, add_feed_to_list, get_feed_list_database};
use crate::types::{errors::CustomError, feeds::FeedMeta};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn tui_loop(open_file: Result<File, Error>) {
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
                            let contents = read_to_string(Path::new("./feed_list.json"))
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
            let read_file = read_to_string(Path::new("./feed_list.json"));
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
        _ => {
            println!("You picked wrong.");
            tui_loop(open_file).await;
        }
    }
}
