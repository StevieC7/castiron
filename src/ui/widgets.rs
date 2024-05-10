use crate::file_handling::feeds::get_feed_list_database;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use iced::widget::{text, Column};
use iced::Element;

#[derive(Clone)]
pub struct Feeds {
    feeds: Vec<FeedMeta>,
}

impl Feeds {
    pub fn new(feeds: Vec<FeedMeta>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        let feeds = self.feeds.to_owned();
        feeds
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(text(content.feed_url.to_owned()))
            })
            .into()
    }
    pub async fn fetch_feeds() -> Result<Vec<FeedMeta>, String> {
        let result = get_feed_list_database();
        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(String::from(format!(
                "Error fetching feeds from database: {:?}",
                e
            ))),
        }
    }
}
