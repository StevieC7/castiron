use crate::{
    file_handling::feeds::get_feed_list_database,
    types::{errors::CustomError, feeds::FeedMeta},
};

use bytes::Bytes;
use reqwest::{get, Error};
use std::{
    fs::OpenOptions,
    io::{copy, Cursor, Seek},
};
use tokio::io::SeekFrom;

pub async fn update_feeds() -> Result<(), CustomError> {
    let feeds = get_feed_list_database()?;
    for feed in feeds {
        match update_single_feed(feed).await {
            Ok(_) => (),
            Err(e) => println!("Error occurred while updating feed {:?}", e),
        }
    }
    Ok(())
}

pub async fn update_single_feed(feed: FeedMeta) -> Result<(), CustomError> {
    let mut updated_feed = get_request(&feed.feed_url).await?;
    let mut xml_file = OpenOptions::new().create(true).write(true).open(
        feed.xml_file_path
            .unwrap_or(format!("./shows/{a}.xml", a = feed.index)),
    )?;
    xml_file.seek(SeekFrom::Start(0))?;
    copy(&mut updated_feed, &mut xml_file)?;
    Ok(())
}

pub async fn get_request(url: &String) -> Result<Cursor<Bytes>, Error> {
    let result = get(url).await?;
    let content = Cursor::new(result.bytes().await?);
    Ok(content)
}
