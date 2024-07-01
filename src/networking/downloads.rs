use reqwest::get;
use std::{
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

use crate::{
    file_handling::episodes::{get_episode_by_id, update_episode_download_status},
    types::errors::CustomError,
};

async fn download_episode(url: &str, file_name: &str) -> Result<String, CustomError> {
    let mut directory = File::create(Path::new(format!("./episodes/{file_name}").as_str()))?;
    let response = get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut directory)?;
    Ok(String::from("Download successful"))
}

pub async fn download_episode_by_guid(id: i32) -> Result<String, CustomError> {
    let episode = get_episode_by_id(id)?;
    download_episode(episode.url.as_str(), episode.file_name.as_str()).await?;
    update_episode_download_status(id, true)?;
    Ok(String::from("Download successful."))
}

pub async fn download_feed_thumbnail(
    url: &str,
    file_extension: &str,
    feed_id: i32,
) -> Result<(), CustomError> {
    let mut file_path = File::create(Path::new(
        format!("./thumbnails/{feed_id}.{file_extension}").as_str(),
    ))?;
    let response = get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut file_path)?;
    Ok(())
}

pub fn check_thumbnail_exists(feed_id: i32, file_extension: &str) -> bool {
    Path::new(format!("./thumbnails/{feed_id}.{file_extension}").as_str()).exists()
}
