use crate::file_handling::feeds::check_feed_exists;
use crate::types::feeds::FeedMeta;

use rand::Rng;
use reqwest::{get, Error};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Seek, Write},
    path::Path,
};
use tokio::io::SeekFrom;

pub fn update_feeds(feeds: Vec<FeedMeta>) {
    for feed in feeds {
        let updated_feed: Result<String, Error> = get_request(&feed.feed_url);
        match updated_feed {
            Ok(_val) => {
                println!("Fetched feed: {:?}", feed.feed_url);
                let mut path_string: String =
                    format!("./shows/{:?}", rand::thread_rng().gen_range(0..10000));
                let path_exists = check_feed_exists(path_string.clone());
                match path_exists {
                    Ok(exists) => {
                        if exists {
                            println!("Path already exists, so we better generate a new one.")
                            // TODO: write the directory name to the list of feeds to reference later
                        } else {
                            println!("Path does not exist, so carry on as usual.");
                            let dir_path: &Path = Path::new(path_string.as_str());
                            let created_dir: Result<(), tokio::io::Error> =
                                create_dir_all(dir_path);
                            match created_dir {
                                Ok(_) => {
                                    path_string.push_str("/feed.xml");
                                    let file_path = Path::new(path_string.as_str());
                                    let xml_file =
                                        OpenOptions::new().create(true).write(true).open(file_path);
                                    match xml_file {
                                        Ok(mut file) => {
                                            let seek_result: Result<u64, tokio::io::Error> =
                                                file.seek(SeekFrom::Start(0));
                                            match seek_result {
                                                Ok(_) => {
                                                    let result = file.write_all(_val.as_bytes());
                                                    // TODO: write the directory name to the list of feeds to reference later
                                                    match result {
                                                    Ok(_) => println!("Successfully created xml file for show: {:?}", feed.feed_url),
                                                    Err(e) => println!("Error writing fetched data to xml file: {e}")
                                                }
                                                }
                                                Err(e) => {
                                                    println!("Error seeking for write head: {e}")
                                                }
                                            }
                                        }
                                        Err(e) => println!("Error creating xml file: {e}"),
                                    }
                                }
                                Err(e) => println!("Error creating directory: {e}"),
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
            Err(e) => println!("Error fetching feed: {e}"),
        }
    }
}

#[tokio::main]
async fn get_request(url: &String) -> Result<String, Error> {
    let result = get(url).await?.text().await?;
    Ok(result)
}
