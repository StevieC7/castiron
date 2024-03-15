use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct FeedMeta {
    pub feed_url: String,
    pub xml_file_path: Option<String>,
}
