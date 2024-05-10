use crate::file_handling::feeds::get_feed_list_database;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use iced::widget::{text, Column};
use iced::Element;

#[derive(Clone)]
pub struct Feeds {
    feeds: Vec<Feed>,
}

impl Feeds {
    pub fn new(feeds: Vec<Feed>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        let col = Column::new();
        let feeds: Element<Message> = self
            .feeds
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(content.view())
            })
            .into();
        col.push(feeds).into()
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

#[derive(Clone)]
pub struct Feed {
    feed_url: String,
}

impl Feed {
    pub fn new(feed_url: String) -> Self {
        Self { feed_url }
    }
    pub fn view(&self) -> Element<Message> {
        text(self.feed_url.to_owned()).into()
    }
}
