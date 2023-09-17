use reqwest::{self, get};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::prelude::*;
use std::{fs, io, path::Path};
use tokio;
use serde::{Serialize, Deserialize};
use rand::Rng;
fn main() {
    let open_file: Option<File> = get_feed_list();
    match open_file {
        Some(ref _file) => {
            let read_file: String = fs::read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
            match read_file.len() {
                0 => println!("Found no feeds to update."),
                _ => {
                    let subscribed_feeds: Result<Vec<FeedMeta>, serde_json::Error> = serde_json::from_str(& read_file);
                    match subscribed_feeds {
                        Ok(feeds) => {
                            update_feeds(feeds)
                        },
                        Err(e) => println!("Error reading feed list: {e}")
                    }
                }
            }
        },
        None => println!("Can't update feeds due to unreadable feed list.")
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
                        Some( _file ) => {
                            let contents = fs::read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file");
                            println!("-----Added successfully, contents below-----\n{contents}\n-------------------");
                        },
                        None => println!("Error saving feed to list.")
                    }
                }
                None => println!("Cannot add if feed list does not exist."),
            }
        }
        "LIST" => {
            println!("------Feeds you are following------");
            let read_file: String = fs::read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
            let contents: Result<Vec<FeedMeta>, serde_json::Error> = serde_json::from_str( & read_file );
            match contents {
                Ok(values) => {
                    for content in values {
                        println!("{}",content.feed_url)
                    }
                },
                Err(e) => println!("{e}")
            }
        },
        // TODO: put a function in to read URLS from a file
        _ => println!("You picked wrong."),
    }
}

#[tokio::main]
async fn get_request(url: &String) -> Result<String, reqwest::Error> {
    let result = get(url).await?.text().await?;
    Ok(result)
}

fn update_feeds(feeds: Vec<FeedMeta>) {
    for feed in feeds {
        let updated_feed: Result<String, reqwest::Error> = get_request(& feed.feed_url);
        match updated_feed {
            Ok(_val) => {
                println!("Fetched feed: {:?}", feed.feed_url);
                let mut path_string: String = format!("./shows/{:?}", rand::thread_rng().gen_range(0..10000));
                let path_exists = check_feed_exists(path_string.clone());
                match path_exists {
                    Ok(exists) => {
                        if exists {
                            println!("Path already exists, so we better generate a new one.")
                            // TODO: write the directory name to the list of feeds to reference later
                        } else {
                            println!("Path does not exist, so carry on as usual.");
                            let dir_path: &Path = Path::new(path_string.as_str());
                            let created_dir: Result<(), io::Error> = create_dir_all(dir_path);
                            match created_dir {
                                Ok(_) => {
                                    path_string.push_str("/feed.xml");
                                    let file_path = Path::new(path_string.as_str());
                                    let xml_file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(file_path);
                                match xml_file {
                                    Ok(mut file) => {
                                        let seek_result: Result<u64, io::Error> = file.seek(io::SeekFrom::Start(0));
                                        match seek_result {
                                            Ok(_) => {
                                                let result = file.write_all(_val.as_bytes());
                                                // TODO: write the directory name to the list of feeds to reference later
                                                match result {
                                                    Ok(_) => println!("Successfully created xml file for show: {:?}", feed.feed_url),
                                                    Err(e) => println!("Error writing fetched data to xml file: {e}")
                                                }
                                            },
                                            Err(e) => println!("Error seeking for write head: {e}")
                                        }
                                    },
                                    Err(e) => println!("Error creating xml file: {e}")
                                }
                            },
                            Err(e) => println!("Error creating directory: {e}")
                        }
                        }
                    },
                    Err(_) => ()
                }
            },
            Err(e) => println!("Error fetching feed: {e}")
        }
    }
}

fn get_feed_list() -> Option<File> {
    let path = Path::new("./feed_list.json");
    let mut file_options = OpenOptions::new();
    let file = file_options
        .create(true)
        .read(true)
        .write(true)
        .open(path);
    match file {
        Ok(file) => Some(file),
        Err(e) => {
            println!("Error finding feed list file: {}", e);
            None
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct FeedMeta {
    feed_url: String,
    xml_file_path: Option<String>
}

fn add_feed_to_list(url: String, mut file: File) -> Option<File> {
    let trimmed_url = url.trim().to_string();
    let feed_meta: FeedMeta = FeedMeta { feed_url: trimmed_url, xml_file_path: None };
    let json_feed = serde_json::to_string(&feed_meta);
    match json_feed {
        Ok( _feed ) => {
            println!("Feed url accepted, attempting to save.");
            let read_file: String = fs::read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
            match read_file.len() {
                0 => {
                    println!("No existing feed list found. Creating now.");
                    let mut vect_feed_seed: Vec<FeedMeta> = Vec::new();
                    vect_feed_seed.push(feed_meta);
                    let vect_feed_seed_string: Result<String, serde_json::Error> = serde_json::to_string(& vect_feed_seed);
                    match vect_feed_seed_string {
                        Ok(byte_string) => {
                            let result: Result<(), io::Error> = file.write_all(byte_string.as_bytes());
                            match result {
                                Ok(_val) => Some(file),
                                Err(e) => {
                                    println!( "{}", e );
                                    None
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error doing serde things: {e}");
                            None
                        }
                    }
                },
                _ => {
                    println!("Existing feed list found.");
                    let existing_json: Result<Vec<FeedMeta>, serde_json::Error> = serde_json::from_str(& read_file);
                    let mut new_json: String = String::new();
                    match existing_json {
                        Ok(mut val) => {
                            val.push(feed_meta);
                            let serialized: Result<String, serde_json::Error> = serde_json::to_string(& val);
                            match serialized {
                                Ok(string) => {
                                    println!("Setting new_json equal to this: {string}");
                                    new_json = string
                                },
                                Err(e) => println!("Error adding feed to accessible file list: {e}")
                            }
                        },
                        Err(e) => println!("Error doing serde stuff: {e}")
                    }
                    let seek_result: Result<u64, io::Error> = file.seek(io::SeekFrom::Start(0));
                    match seek_result {
                        Ok(_) => {
                            let result = Some(file.write_all(new_json.as_bytes()));
                            match result {
                                Some(Ok(_val)) => Some(file),
                                Some(Err(e)) => {
                                    println!("Error writing to file: {e}");
                                    None
                                },
                                None => {
                                    None
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error seeking to beginning of file: {e}");
                            None
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("An error occurred: {e}");
            None
        }
    }
}

fn check_feed_exists (comparison_path: String) -> Result<bool, serde_json::Error> {
    let read_file: String = fs::read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
    let contents: Result<Vec<FeedMeta>, serde_json::Error> = serde_json::from_str( & read_file );
    match contents {
        Ok(values) => {
            if values.len() == 0 {
                return Ok(false);
            } else {
                for content in values {
                    match content.xml_file_path {
                        Some(val) => {
                            if val == comparison_path {
                                return Ok(true);
                            }
                        },
                        None => ()
                    }
                }
                return Ok(false);
            }
        },
        Err(e) => Err(e)
    }
}