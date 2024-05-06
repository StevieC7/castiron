use reqwest::get;
use roxmltree::Document;
use std::{
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

use crate::{
    file_handling::feeds::{check_episode_exists, load_feeds_xml},
    types::errors::CustomError,
};

//
// parse list of feeds
// check which episodes have been downloaded
// check user specified limit for episodes kept
// if there is room within the user's limit and filesystem, download the episodes that have not been downloaded
//

// TODO: refactor to obtain as many of latest episodes as user specifies
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
                                        } else {
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
    println!("Downloading: {:?} /n from {:?}", file_name, url);
    let mut directory = File::create(Path::new(format!("./episodes/{file_name}").as_str()))?;
    let response = get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut directory)?;
    Ok(String::from("Download successful"))
}
