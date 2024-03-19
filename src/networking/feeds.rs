use crate::types::feeds::FeedMeta;

use bytes::Bytes;
use reqwest::{get, Error};
use std::{
    fs::OpenOptions,
    io::{copy, Cursor, Seek},
};
use tokio::io::SeekFrom;

pub async fn update_feeds(feeds: Vec<FeedMeta>) {
    for feed in feeds {
        let updated_feed: Result<Cursor<Bytes>, Error> = get_request(&feed.feed_url).await;
        match updated_feed {
            Ok(mut val) => {
                println!("Fetched feed: {:?}", feed.feed_url);
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
                                match result {
                                    Ok(_) => {
                                        println!(
                                            "Successfully created xml file for show: {:?}",
                                            feed.feed_url
                                        )
                                    }
                                    Err(e) => {
                                        println!("Error writing fetched data to xml file: {e}")
                                    }
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
            Err(e) => println!("Error fetching feed: {e}"),
        }
    }
}

async fn get_request(url: &String) -> Result<Cursor<Bytes>, Error> {
    let result = get(url).await?;
    let content = Cursor::new(result.bytes().await?);
    Ok(content)
}
