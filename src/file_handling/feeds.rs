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

pub fn add_feed_to_list(url: String, mut file: File) -> Option<File> {
    let trimmed_url = url.trim().to_string();
    let feed_meta: FeedMeta = FeedMeta {
        feed_url: trimmed_url,
        xml_file_path: None,
    };
    let json_feed = to_string(&feed_meta);
    match json_feed {
        Ok(_feed) => {
            println!("Feed url accepted, attempting to save.");
            let read_file: String =
                read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
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
                                file.write_all(byte_string.as_bytes());
                            match result {
                                Ok(_val) => Some(file),
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
                    let seek_result: Result<u64, IOError> = file.seek(SeekFrom::Start(0));
                    match seek_result {
                        Ok(_) => {
                            let result = Some(file.write_all(new_json.as_bytes()));
                            match result {
                                Some(Ok(_val)) => Some(file),
                                Some(Err(e)) => {
                                    println!("Error writing to file: {e}");
                                    None
                                }
                                None => None,
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

pub fn check_feed_exists(comparison_path: String) -> Result<bool, SerdeError> {
    let read_file: String =
        read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
    let contents: Result<Vec<FeedMeta>, SerdeError> = from_str(&read_file);
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
                        }
                        None => (),
                    }
                }
                return Ok(false);
            }
        }
        Err(e) => Err(e),
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
                let feed_xml_file = read_dir(directory.path());
                match feed_xml_file {
                    Ok(mut xml_file) => {
                        let file_path = xml_file.next();
                        match file_path {
                            Some(thing) => match thing {
                                Ok(final_path) => {
                                    let actual_path = final_path.path();
                                    let feed_content = read_to_string(actual_path);
                                    match feed_content {
                                        Ok(content) => {
                                            feed_collection.push(content);
                                        }
                                        Err(e) => println!("{e} at feed_content match"),
                                    }
                                }
                                Err(e) => println!("{e} at file_path match"),
                            },
                            None => (),
                        }
                    }
                    Err(e) => println!("{e} at feed_xml_file match"),
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
