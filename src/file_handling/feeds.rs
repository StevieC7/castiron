use crate::types::feeds::FeedMeta;
use serde_json::{from_str, to_string, Error as SerdeError};
use std::{
    fs::{read_dir, read_to_string, File, OpenOptions},
    io::{Error as IOError, Seek, SeekFrom, Write},
    path::Path,
};

pub fn get_feed_list() -> Option<File> {
    let path = Path::new("./feed_list.json");
    let mut file_options = OpenOptions::new();
    let file = file_options.create(true).read(true).write(true).open(path);
    match file {
        Ok(file) => Some(file),
        Err(e) => {
            println!("Error finding feed list file: {}", e);
            None
        }
    }
}

pub fn add_feed_to_list(url: String, mut feed_list_file: File) -> Option<File> {
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
    let this_feed_index = existing_list_highest_index + 1;
    let path_string: String = format!("./shows/{:?}.xml", this_feed_index);
    let feed_meta: FeedMeta = FeedMeta {
        index: this_feed_index,
        feed_url: trimmed_url,
        xml_file_path: Some(path_string),
    };
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

pub fn load_feeds_xml() -> Option<Vec<String>> {
    println!("Loading feeds xml");
    let feed_path_list =
        read_dir("./shows").expect("Show directory is missing or improperly formatted.");

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
    Some(feed_collection)
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
