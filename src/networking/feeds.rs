use crate::{file_handling::feeds::get_feed_list_database, types::errors::CustomError};

use bytes::Bytes;
use reqwest::{get, Error};
use std::{
    fs::OpenOptions,
    io::{copy, Cursor, Seek},
};
use tokio::io::SeekFrom;

pub async fn update_feeds() -> Result<(), CustomError> {
    // TODO: refactor so that individual feeds don't cause whole operation to fail, excluding valid feeds from succeeding
    let feeds = get_feed_list_database()?;
    for feed in feeds {
        let mut updated_feed = get_request(&feed.feed_url).await?;
        println!("Fetched feed: {:?}", feed.feed_url);
        let mut xml_file = OpenOptions::new().create(true).write(true).open(
            feed.xml_file_path
                .unwrap_or(format!("./shows/{a}.xml", a = feed.index)),
        )?;
        xml_file.seek(SeekFrom::Start(0))?;
        copy(&mut updated_feed, &mut xml_file)?;
        println!("Successfully created xml file for show: {}", feed.feed_url)
    }
    Ok(())
}

pub async fn get_request(url: &String) -> Result<Cursor<Bytes>, Error> {
    let result = get(url).await?;
    let content = Cursor::new(result.bytes().await?);
    Ok(content)
}
