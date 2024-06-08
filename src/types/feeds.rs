use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedMeta {
    pub id: i32,
    pub feed_url: String,
    pub xml_file_path: Option<String>,
    pub feed_title: Option<String>,
}
