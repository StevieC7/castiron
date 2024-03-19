use crate::types::feeds::FeedMeta;
use serde_json::{from_str, to_string, Error as SerdeError};
use std::{
    fs::{self, read_dir, read_to_string, File, OpenOptions},
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
    let existing_list_length = match existing_list {
        Ok(list) => list.len(),
        Err(_) => 0,
    };
    let path_string: String = format!("./shows/{:?}.xml", existing_list_length + 1);
    let feed_meta: FeedMeta = FeedMeta {
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

pub fn add_xml_path_to_feed_list(feed_url: &str, xml_path: &Path) -> Result<(), IOError> {
    let read_file =
        read_to_string(Path::new("./feed_list.json")).expect("Oopsie reading saved file.");
    let contents: Result<Vec<FeedMeta>, SerdeError> = from_str(&read_file);
    let mut new_json: String = String::new();
    match contents {
        Ok(mut feed_list) => {
            let position_of_existing = feed_list.iter().position(|x| x.feed_url == feed_url);
            match position_of_existing {
                Some(pos) => {
                    let existing_url = &feed_list[pos].feed_url;
                    feed_list[pos] = FeedMeta {
                        feed_url: existing_url.to_string(),
                        xml_file_path: Some(xml_path.to_string_lossy().to_string()),
                    };
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .open(Path::new("./feed_list.json"))
                        .expect("Could not open feed list to save xml path.");
                    let serialized: Result<String, SerdeError> = to_string(&feed_list);
                    match serialized {
                        Ok(string) => {
                            println!("Setting new_json equal to this: {string}");
                            new_json = string
                        }
                        Err(e) => {
                            println!("Error adding feed to accessible file list: {e}")
                        }
                    }
                    let seek_result = file.seek(SeekFrom::Start(0));
                    match seek_result {
                        Ok(_) => {
                            let result = file.write_all(new_json.as_bytes());
                            match result {
                                Ok(_) => return Ok(()),
                                Err(e) => {
                                    println!("Error line 208 feeds feed_handling");
                                    return Err(e);
                                }
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                None => (),
            }
        }
        Err(e) => println!("{e}"),
    }
    Ok(())
}
