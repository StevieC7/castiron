use reqwest::get;
use roxmltree::Document;
use std::{
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

use crate::{
    file_handling::{
        episodes::{
            add_episode_to_database, get_episode_by_guid, get_episode_list_database,
            update_episode_download_status,
        },
        feeds::{get_feed_list_database, load_feed_xml, update_feed_title},
    },
    types::{episodes::Episode, errors::CustomError},
};

use super::feeds::update_feeds;

pub async fn sync_episode_list() -> Result<Option<Vec<Episode>>, CustomError> {
    update_feeds().await?;
    let feed_collection = get_feed_list_database()?;
    let mut episodes: Vec<Episode> = Vec::new();
    for feed in feed_collection {
        let content = load_feed_xml(feed.xml_file_path.unwrap_or(String::new()))?;
        let feed_contents = content.as_str();
        let doc = Document::parse(feed_contents)?;
        let channel_node = doc.descendants().find(|n| n.has_tag_name("channel"));
        match channel_node {
            Some(c_node) => {
                let title_node = c_node.descendants().find(|n| n.has_tag_name("title"));
                match title_node {
                    Some(title) => {
                        match update_feed_title(feed.id, title.text().unwrap().to_string()) {
                            _ => {}
                        }
                    }
                    None => {}
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
                                println!("No url found for {:?}.", g_node.text())
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

async fn download_episode(url: &str, file_name: &str) -> Result<String, CustomError> {
    println!("Downloading: {:?} from {:?}", file_name, url);
    let mut directory = File::create(Path::new(format!("./episodes/{file_name}").as_str()))?;
    let response = get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut directory)?;
    Ok(String::from("Download successful"))
}

pub async fn download_episode_by_guid(id: i32) -> Result<String, CustomError> {
    let episode = get_episode_by_guid(id)?;
    println!("DEBUG: retrieved {:?}", episode);
    download_episode(episode.url.as_str(), episode.file_name.as_str()).await?;
    update_episode_download_status(id, true)?;
    Ok(String::from("Download successful."))
}
