use reqwest::{get, Error as ReqwestError};
use roxmltree::Document;
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
    let doc = Document::parse("<title>You Suck</title>");
    match doc {
        Ok(stuff) => {
            let ready_val = stuff.descendants().find(|n| n.has_tag_name("title"))?;
            match ready_val.text() {
                Some(thing) => {
                    println!("{thing}");
                    Some(thing.to_string())
                }
                None => None,
            }
        }
        Err(e) => {
            println!("{e}");
            None
        }
    }
}

#[tokio::main]
async fn get_request(url: &String) -> Result<String, ReqwestError> {
    let result = get(url).await?.text().await?;
    Ok(result)
}
