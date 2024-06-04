use reqwest::get;
use roxmltree::Document;
use std::{
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

use crate::{
    file_handling::{
        episodes::{add_episode_to_database, get_episode_by_guid, get_episode_list_database},
        feeds::{check_episode_exists, get_feed_id_by_url, load_feeds_xml},
    },
    types::{episodes::Episode, errors::CustomError},
};

use super::feeds::update_feeds;

pub async fn sync_episode_list() -> Result<Option<Vec<Episode>>, CustomError> {
    update_feeds().await?;
    let feed_collection = load_feeds_xml()?;
    let mut episodes: Vec<Episode> = Vec::new();
    for feed in feed_collection {
        let feed_contents = feed.as_str();
        let mut feed_url = String::new();
        let doc = Document::parse(feed_contents)?;
        let channel_node = doc.descendants().find(|n| n.has_tag_name("channel"));
        match channel_node {
            Some(c_node) => {
                let link_node = c_node.descendants().find(|n| n.has_tag_name("atom:link"));
                match link_node {
                    Some(link) => feed_url = link.text().unwrap().to_string(),
                    None => (),
                }
            }
            None => (),
        }
        let feed_id = get_feed_id_by_url(&feed_url)?;
        let item_node = doc.descendants().find(|n| n.has_tag_name("item"));
        match item_node {
            Some(i_node) => {
                let title_node = i_node.descendants().find(|n| n.has_tag_name("title"));
                let episode_title = match title_node {
                    Some(t) => t.text().unwrap(),
                    None => "",
                };
                let date_node = i_node.descendants().find(|n| n.has_tag_name("pubDate"));
                let episode_date = match date_node {
                    Some(d) => d.text().unwrap(),
                    None => "",
                };
                let guid_node = i_node.descendants().find(|n| n.has_tag_name("guid"));
                match guid_node {
                    Some(g_node) => {
                        let guid = g_node.text().unwrap();
                        let enclosure_node =
                            i_node.descendants().find(|n| n.has_tag_name("enclosure"));
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
                                        guid: guid.to_string(),
                                        file_name,
                                        title: episode_title.to_string(),
                                        date: episode_date.to_string(),
                                        played: false,
                                        played_seconds: 0,
                                        feed_id,
                                        url: url.to_string(),
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
            None => println!("got no node"),
        }
    }
    // TODO: write bulk upsert function to use here
    for episode in episodes.into_iter() {
        add_episode_to_database(episode)?;
    }
    let result = get_episode_list_database()?;
    Ok(Some(result))
}

//
// parse list of feeds
// check which episodes have been downloaded
// check user specified limit for episodes kept
// if there is room within the user's limit and filesystem, download the episodes that have not been downloaded
//
// TODO: refactor to obtain as many of latest episodes as user specifies
// TODO: refactor to use database as source of truth
pub async fn download_episodes() -> Result<(), CustomError> {
    let feed_collection = load_feeds_xml().unwrap_or(Vec::new());
    for feed in feed_collection {
        let feed_contents = feed.as_str();
        let doc = Document::parse(feed_contents);
        match doc {
            Ok(stuff) => {
                let item_node = stuff.descendants().find(|n| n.has_tag_name("item"));
                match item_node {
                    Some(i_node) => {
                        let guid_node = i_node.descendants().find(|n| n.has_tag_name("guid"));
                        match guid_node {
                            Some(g_node) => {
                                let enclosure_node =
                                    i_node.descendants().find(|n| n.has_tag_name("enclosure"));
                                match enclosure_node {
                                    Some(e_node) => {
                                        let guid = g_node.text().unwrap();
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
                                        if let Ok(true) = check_episode_exists(file_name.as_str()) {
                                            println!("Episode already exists {:?}", file_name);
                                            continue;
                                        }
                                        match e_node.attribute("url") {
                                            Some(url) => {
                                                let download_result =
                                                    download_episode(url, file_name.as_str()).await;
                                                match download_result {
                                                    Ok(_result) => (),
                                                    Err(e) => println!("Download function {:?}", e),
                                                }
                                            }
                                            None => {
                                                println!("No url found for {:?}.", g_node.text())
                                            }
                                        }
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    }
                    None => println!("got no node"),
                }
            }
            Err(e) => {
                println!("{e}");
                ()
            }
        }
    }
    Ok(())
}

async fn download_episode(url: &str, file_name: &str) -> Result<String, CustomError> {
    println!("Downloading: {:?} from {:?}", file_name, url);
    let mut directory = File::create(Path::new(format!("./episodes/{file_name}").as_str()))?;
    let response = get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut directory)?;
    Ok(String::from("Download successful"))
}

pub async fn download_episode_by_guid(guid: String) -> Result<String, CustomError> {
    let episode = get_episode_by_guid(guid)?;
    println!("DEBUG: retrieved {:?}", episode);
    download_episode(episode.url.as_str(), episode.file_name.as_str()).await?;
    Ok(String::from("Download successful."))
}
