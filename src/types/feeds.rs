use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedMeta {
    pub index: i32,
    pub feed_url: String,
    pub xml_file_path: Option<String>,
}
