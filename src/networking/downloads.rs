use reqwest::{get, Error as ReqwestError};
use roxmltree::Document;
use std::fs;
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
pub fn download_episodes() -> Option<String> {
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
        let fmt_string = feed.as_str();
        let doc = Document::parse(fmt_string);
        match doc {
            Ok(stuff) => {
                let ready_val = stuff.descendants().find(|n| n.has_tag_name("title"))?;
                match ready_val.text() {
                    Some(thing) => {
                        println!("{thing}");
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

#[tokio::main]
async fn get_request(url: &String) -> Result<String, ReqwestError> {
    let result = get(url).await?.text().await?;
    Ok(result)
}
