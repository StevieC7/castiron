use reqwest::{get, Error};
use tokio;

use crate::types::feeds::FeedMeta;
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
pub fn download_episodes(feeds: Vec<FeedMeta>) {}

#[tokio::main]
async fn get_request(url: &String) -> Result<String, Error> {
    let result = get(url).await?.text().await?;
    Ok(result)
}
