use crate::{
    file_handling::{
        episodes::{
            add_episode_to_database, get_episode_list_database,
            mark_episodes_deleted_if_file_nonexistent,
        },
        feeds::{
            get_feed_list_database, load_feed_xml, update_feed_file_path, update_feed_title,
            update_thumbnail_file_path,
        },
    },
    types::{episodes::Episode, errors::CustomError, feeds::FeedMeta},
};

use bytes::Bytes;
use reqwest::{get, Error};
use roxmltree::Document;
use std::{
    fs::OpenOptions,
    io::{copy, Cursor, Seek},
};
use tokio::io::SeekFrom;
use url::{Position, Url};

use super::downloads::{check_thumbnail_exists, download_feed_thumbnail};

pub async fn sync_episode_list() -> Result<Option<Vec<Episode>>, CustomError> {
    update_feeds().await?;
    mark_episodes_deleted_if_file_nonexistent()?;
    let feed_collection = get_feed_list_database()?;
    let mut episodes: Vec<Episode> = Vec::new();
    for feed in feed_collection {
        let content = load_feed_xml(feed.xml_file_path.unwrap_or(String::new()))?;
        let feed_contents = content.as_str();
        let doc = Document::parse(feed_contents)?;
        let channel_node = doc.descendants().find(|n| n.has_tag_name("channel"));
        match channel_node {
            Some(c_node) => {
                if let Some(title_node) = c_node.descendants().find(|n| n.has_tag_name("title")) {
                    update_feed_title(feed.id, title_node.text().unwrap().to_string())?;
                }
                if let Some(image_node) = c_node.descendants().find(|n| n.has_tag_name("image")) {
                    if let Some(url_node) = image_node.descendants().find(|n| n.has_tag_name("url"))
                    {
                        if let Some(url) = url_node.text() {
                            if let Ok(file_extension) = parse_file_extension_from_image_url(url) {
                                if !check_thumbnail_exists(feed.id, file_extension.as_str()) {
                                    download_feed_thumbnail(url, file_extension.as_str(), feed.id)
                                        .await?;
                                    update_thumbnail_file_path(
                                        feed.id,
                                        format!("./thumbnails/{}.{file_extension}", feed.id),
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
            None => (),
        }
        let episode_nodes = doc.descendants().filter(|n| n.has_tag_name("item"));
        for e_node in episode_nodes {
            let title_node = e_node.descendants().find(|n| n.has_tag_name("title"));
            let episode_title = match title_node {
                Some(t) => t.text().unwrap(),
                None => "",
            };
            let date_node = e_node.descendants().find(|n| n.has_tag_name("pubDate"));
            let episode_date = match date_node {
                Some(d) => d.text().unwrap(),
                None => "",
            };
            let guid_node = e_node.descendants().find(|n| n.has_tag_name("guid"));
            match guid_node {
                Some(g_node) => {
                    let guid = g_node.text().unwrap();
                    let enclosure_node = e_node.descendants().find(|n| n.has_tag_name("enclosure"));
                    match enclosure_node {
                        Some(e_node) => match e_node.attribute("url") {
                            Some(url) => {
                                let file_name = match e_node.attribute("type") {
                                    Some("audio/aac") => format!("{guid}.aac"),
                                    Some("audio/mpeg") => format!("{guid}.mp3"),
                                    Some("audio/ogg") => format!("{guid}.oga"),
                                    Some("audio/opus") => format!("{guid}.opus"),
                                    Some("audio/wav") => format!("{guid}.wav"),
                                    Some("audio/webm") => format!("{guid}.weba"),
                                    Some(_) => format!("{guid}.mp3"),
                                    None => "fail.mp3".to_string(),
                                };
                                episodes.push(Episode {
                                    id: 0,
                                    guid: guid.to_string(),
                                    file_name,
                                    title: episode_title.to_string(),
                                    date: episode_date.to_string(),
                                    played: false,
                                    played_seconds: 0,
                                    feed_id: feed.id,
                                    url: url.to_string(),
                                    downloaded: false,
                                })
                            }
                            None => {
                                eprintln!("No url found for {:?}.", g_node.text())
                            }
                        },
                        None => (),
                    }
                }
                None => (),
            }
        }
    }
    for episode in episodes.into_iter() {
        add_episode_to_database(episode)?;
    }
    let result = get_episode_list_database()?;
    Ok(Some(result))
}

pub async fn update_feeds() -> Result<(), CustomError> {
    let feeds = get_feed_list_database()?;
    for feed in feeds {
        match update_single_feed(feed).await {
            Ok(_) => (),
            Err(e) => eprintln!("Error occurred while updating feed {:?}", e),
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

pub fn parse_file_extension_from_image_url(url: &str) -> Result<String, CustomError> {
    let parsed_url = Url::parse(url)?;
    let through_path_only = &parsed_url[..Position::AfterPath];
    Ok(through_path_only
        .get(through_path_only.len() - 3..through_path_only.len())
        .unwrap_or("jpg")
        .to_string())
}
