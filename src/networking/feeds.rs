use crate::types::feeds::FeedMeta;

use bytes::Bytes;
use reqwest::{get, Error};
use std::{
    fs::{read_dir, OpenOptions},
    io::{copy, Cursor, Seek},
    path::Path,
};
use tokio::io::SeekFrom;

pub async fn update_feeds(feeds: Vec<FeedMeta>) {
    for feed in feeds {
        let updated_feed: Result<Cursor<Bytes>, Error> = get_request(&feed.feed_url).await;
        match updated_feed {
            Ok(mut val) => {
                println!("Fetched feed: {:?}", feed.feed_url);
                let directory_contents = read_dir(Path::new("./shows"));
                let existing_path = feed.xml_file_path.as_ref().unwrap();
                println!("{:?}{:?}", directory_contents, existing_path);
                let path_exists: bool;
                match directory_contents {
                    Ok(read_dir) => {
                        let mut entries = read_dir.map(|x| x.ok().unwrap());
                        let find_result = entries
                            .find(|x| x.path().to_string_lossy() == existing_path.to_owned());
                        println!("{:?}", find_result);
                        match find_result {
                            Some(_) => path_exists = true,
                            None => path_exists = false,
                        }
                    }
                    Err(_) => path_exists = false,
                }
                match path_exists {
                    true => {
                        // TODO: update in place
                        println!("Path already exists, so we need to update it in place.");
                    }
                    false => {
                        println!("Path does not exist, so carry on as usual.");
                        let xml_file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .open(feed.xml_file_path.unwrap());
                        match xml_file {
                            Ok(mut file) => {
                                let seek_result: Result<u64, tokio::io::Error> =
                                    file.seek(SeekFrom::Start(0));
                                match seek_result {
                                    Ok(_) => {
                                        let result = copy(&mut val, &mut file);
                                        // TODO: write the directory name to the list of feeds to reference later
                                        match result {
                                            Ok(_) => {
                                                println!(
                                                    "Successfully created xml file for show: {:?}",
                                                    feed.feed_url
                                                )
                                            }
                                            Err(e) => println!(
                                                "Error writing fetched data to xml file: {e}"
                                            ),
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
                }
            }
            Err(e) => println!("Error fetching feed: {e}"),
        }
    }
}

async fn get_request(url: &String) -> Result<Cursor<Bytes>, Error> {
    let result = get(url).await?;
    let content = Cursor::new(result.bytes().await?);
    Ok(content)
}
