use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct FeedMeta {
    pub index: usize,
    pub feed_url: String,
    pub xml_file_path: Option<String>,
}
