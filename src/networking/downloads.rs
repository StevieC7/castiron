use reqwest::{get, Error as ReqwestError};
use roxmltree::Document;
use std::{
    fs::{self, File},
    io::{copy, Error},
    path::Path,
};
use tokio;

//
// parse list of feeds
// check which episodes have been downloaded
// check user specified limit for episodes kept
// if there is room within the user's limit and filesystem, download the episodes that have not been downloaded
//

//
// parse list of feeds
// download the single latest episode
//
#[tokio::main]
pub async fn download_episodes() -> Option<String> {
    // iterate over existing feeds in /shows directory
    let feed_path_list =
        fs::read_dir("./shows").expect("Show directory is missing or improperly formatted.");

    let mut feed_collection = Vec::new();
    for feed in feed_path_list {
        match feed {
            Ok(directory) => {
                let feed_xml_file = fs::read_dir(directory.path());
                match feed_xml_file {
                    Ok(mut xml_file) => {
                        let file_path = xml_file.next();
                        match file_path {
                            Some(thing) => match thing {
                                Ok(final_path) => {
                                    let actual_path = final_path.path();
                                    let feed_content = fs::read_to_string(actual_path);
                                    match feed_content {
                                        Ok(content) => {
                                            feed_collection.push(content);
                                        }
                                        Err(e) => println!("{e}"),
                                    }
                                }
                                Err(e) => println!("{e}"),
                            },
                            None => (),
                        }
                    }
                    Err(e) => println!("{e}"),
                }
            }
            Err(e) => println!("{e}"),
        }
    }
    println!("{:?}", feed_collection.len());
    for feed in feed_collection {
        let feed_contents = feed.as_str();
        let doc = Document::parse(feed_contents);
        match doc {
            Ok(stuff) => {
                let item_node = stuff.descendants().find(|n| n.has_tag_name("item"))?;
                let title_node = item_node.descendants().find(|n| n.has_tag_name("title"))?;
                let enclosure_node = item_node
                    .descendants()
                    .find(|n| n.has_tag_name("enclosure"))?;
                let title = title_node.text().unwrap();
                match enclosure_node.attribute("url") {
                    Some(url) => {
                        // put stuff here
                        let download_result = download_episode(url, title).await;
                        match download_result {
                            Ok(result) => println!("Download function {result}"),
                            Err(e) => println!("Download function {:?}", e),
                        }
                    }
                    None => (),
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
    let mut directory = File::create(Path::new(format!("./episodes/{file_name}").as_str()))?;
    let response = get(url).await?;
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut directory)?;
    Ok(String::from("Download successful"))
}
