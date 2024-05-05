use crate::networking::feeds::get_request;
use crate::types::feeds::FeedMeta;
use roxmltree::Document;
use serde_json::{from_str, to_string, Error as SerdeError};
use std::{
    fs::{read_dir, read_to_string, File, OpenOptions},
    io::{read_to_string as read_cursor_to_string, Error as IOError, Seek, SeekFrom, Write},
    path::Path,
};

pub fn get_feed_list() -> Result<File, IOError> {
    let path = Path::new("./feed_list.json");
    let mut file_options = OpenOptions::new();
    let file = file_options
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;
    Ok(file)
}

pub async fn add_feed_to_list(url: String, mut feed_list_file: File) -> Option<File> {
    let trimmed_url = url.trim().to_string();
    let read_file =
        read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
    let existing_list: Result<Vec<FeedMeta>, SerdeError> = from_str(read_file.as_str());
    let existing_list_highest_index = match existing_list {
        Ok(list) => {
            let mut indices = list.into_iter().map(|x| x.index).collect::<Vec<usize>>();
            indices.sort();
            indices.last().unwrap().to_owned()
        }
        Err(_) => 0,
    };
    // TODO: show the user a preview of parsed show title and confirm before adding
    let this_feed_index = existing_list_highest_index + 1;
    let path_string: String = format!("./shows/{:?}.xml", this_feed_index);
    let mut feed_meta: FeedMeta = FeedMeta {
        index: this_feed_index,
        feed_url: trimmed_url.to_owned(),
        xml_file_path: Some(path_string),
    };

    if let Ok(feed_content_reader) = get_request(&trimmed_url).await {
        if let Ok(feed_content) = read_cursor_to_string(feed_content_reader) {
            if let Ok(parsed_feed) = Document::parse(feed_content.as_str()) {
                if let Some(channel) = parsed_feed
                    .descendants()
                    .find(|n| n.has_tag_name("channel"))
                {
                    if let Some(title) = channel.descendants().find(|n| n.has_tag_name("title")) {
                        feed_meta.xml_file_path =
                            Some(format!("./shows/{:?}.xml", title.text().unwrap()))
                    }
                } else {
                    println!("A deserialization error occurred while fetching feed preview.")
                }
            } else {
                println!("A deserialization error occurred while fetching feed preview.")
            }
        } else {
            println!("An error occurred while fetching feed preview.")
        }
    } else {
        println!("An error occurred while fetching feed preview.")
    }

    let json_feed = to_string(&feed_meta);
    match json_feed {
        Ok(_) => {
            println!("Feed url accepted, attempting to save.");
            match read_file.len() {
                0 => {
                    println!("No existing feed list found. Creating now.");
                    let mut vect_feed_seed: Vec<FeedMeta> = Vec::new();
                    vect_feed_seed.push(feed_meta);
                    let vect_feed_seed_string: Result<String, SerdeError> =
                        to_string(&vect_feed_seed);
                    match vect_feed_seed_string {
                        Ok(byte_string) => {
                            let result: Result<(), IOError> =
                                feed_list_file.write_all(byte_string.as_bytes());
                            match result {
                                Ok(_val) => Some(feed_list_file),
                                Err(e) => {
                                    println!("{}", e);
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error doing serde things: {e}");
                            None
                        }
                    }
                }
                _ => {
                    println!("Existing feed list found.");
                    let existing_json: Result<Vec<FeedMeta>, SerdeError> = from_str(&read_file);
                    let mut new_json: String = String::new();
                    match existing_json {
                        Ok(mut val) => {
                            val.push(feed_meta);
                            let serialized: Result<String, SerdeError> = to_string(&val);
                            match serialized {
                                Ok(string) => {
                                    println!("Setting new_json equal to this: {string}");
                                    new_json = string
                                }
                                Err(e) => {
                                    println!("Error adding feed to accessible file list: {e}")
                                }
                            }
                        }
                        Err(e) => println!("Error doing serde stuff: {e}"),
                    }
                    let seek_result: Result<u64, IOError> = feed_list_file.seek(SeekFrom::Start(0));
                    match seek_result {
                        Ok(_) => {
                            let result = feed_list_file.write_all(new_json.as_bytes());
                            match result {
                                Ok(_val) => {
                                    println!("Wrote to file successfully");
                                    Some(feed_list_file)
                                }
                                Err(e) => {
                                    println!("Error writing to file: {e}");
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error seeking to beginning of file: {e}");
                            None
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("An error occurred: {e}");
            None
        }
    }
}

pub fn load_feeds_xml() -> Result<Vec<String>, IOError> {
    println!("Loading feeds xml");
    let feed_path_list = read_dir("./shows")?;

    let mut feed_collection = Vec::new();
    for feed in feed_path_list {
        match feed {
            Ok(directory) => {
                let feed_content = read_to_string(directory.path());
                match feed_content {
                    Ok(content) => feed_collection.push(content),
                    Err(e) => println!("{e}"),
                }
            }
            Err(e) => println!("{e}"),
        }
    }
    Ok(feed_collection)
}

pub fn check_episode_exists(file_name: &str) -> bool {
    let episode_list = read_dir("./episodes");
    match episode_list {
        Ok(mut episodes) => {
            let found_existing = episodes.find(|episode| {
                let directory_entry = episode.as_ref().ok();
                match directory_entry {
                    Some(entry) => {
                        if entry.file_name() == file_name {
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                }
            });
            match found_existing {
                Some(_) => true,
                None => false,
            }
        }
        Err(_) => false,
    }
}
