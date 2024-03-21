use reqwest::{get, Error as ReqwestError};
use roxmltree::Document;
use std::{
    fs::File,
    io::{copy, Cursor, Error},
    path::Path,
};

use crate::file_handling::feeds::{check_episode_exists, load_feeds_xml};

//
// parse list of feeds
// check which episodes have been downloaded
// check user specified limit for episodes kept
// if there is room within the user's limit and filesystem, download the episodes that have not been downloaded
//

// TODO: refactor to obtain as many of latest episodes as user specifies
pub async fn download_episodes() -> Option<String> {
    // iterate over existing feeds in /shows directory
    let feed_collection = load_feeds_xml().unwrap_or(Vec::new());
    for feed in feed_collection {
        let feed_contents = feed.as_str();
        let doc = Document::parse(feed_contents);
        match doc {
            Ok(stuff) => {
                let item_node = stuff.descendants().find(|n| n.has_tag_name("item"));
                match item_node {
                    Some(i_node) => {
                        let guid_node = i_node.descendants().find(|n| n.has_tag_name("guid"))?;
                        let enclosure_node =
                            i_node.descendants().find(|n| n.has_tag_name("enclosure"))?;
                        let guid = guid_node.text().unwrap();
                        let file_name = match enclosure_node.attribute("type") {
                            Some("audio/aac") => format!("{guid}.aac"),
                            Some("audio/mpeg") => format!("{guid}.mp3"),
                            Some("audio/ogg") => format!("{guid}.oga"),
                            Some("audio/opus") => format!("{guid}.opus"),
                            Some("audio/wav") => format!("{guid}.wav"),
                            Some("audio/webm") => format!("{guid}.weba"),
                            Some(_) => format!("{guid}.mp3"),
                            None => "fail.mp3".to_string(),
                        };
                        if check_episode_exists(file_name.as_str()) {
                            println!("Episode already exists {:?}", file_name);
                            continue;
                        }
                        match enclosure_node.attribute("url") {
                            Some(url) => {
                                let download_result =
                                    download_episode(url, file_name.as_str()).await;
                                match download_result {
                                    Ok(result) => println!("Download function {result}"),
                                    Err(e) => println!("Download function {:?}", e),
                                }
                            }
                            None => continue,
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
    None
}

#[derive(Debug)]
enum MultiError {
    Error1(Error),
    Error2(ReqwestError),
}

impl From<Error> for MultiError {
    fn from(e: Error) -> Self {
        MultiError::Error1(e)
    }
}
impl From<ReqwestError> for MultiError {
    fn from(e: ReqwestError) -> Self {
        MultiError::Error2(e)
    }
}

async fn download_episode(url: &str, file_name: &str) -> Result<String, MultiError> {
    println!("Downloading: {:?} /n from {:?}", file_name, url);
    let mut directory = File::create(Path::new(format!("./episodes/{file_name}").as_str()))?;
    let response = get(url).await?;
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut directory)?;
    Ok(String::from("Download successful"))
}
