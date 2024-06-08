use crate::{
    file_handling::feeds::{get_feed_list_database, update_feed_file_path},
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
    let mut should_set_xml_path_equal_id = false;
    let mut updated_feed = get_request(&feed.feed_url).await?;
    let mut xml_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(feed.xml_file_path.unwrap_or_else(|| {
            // TODO: update the feed xml_fil_path to equal the feed id
            should_set_xml_path_equal_id = true;
            format!("./shows/{a}.xml", a = feed.id)
        }))?;
    xml_file.seek(SeekFrom::Start(0))?;
    copy(&mut updated_feed, &mut xml_file)?;
    if should_set_xml_path_equal_id {
        update_feed_file_path(feed.id, format!("./shows/{a}.xml", a = feed.id))?;
    }
    Ok(())
}

pub async fn get_request(url: &String) -> Result<Cursor<Bytes>, Error> {
    let result = get(url).await?;
    let content = Cursor::new(result.bytes().await?);
    Ok(content)
}
