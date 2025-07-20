use super::feed::Feed;
use crate::{
    file_handling::feeds::get_feed_list_database, types::feeds::FeedMeta, ui::gui::Message,
};
use iced::{
    alignment::Horizontal,
    widget::{container, text, Column, Scrollable},
    Element, Length,
};

pub struct FeedList {
    feeds: Vec<Feed>,
}

impl FeedList {
    pub fn new(feeds: Vec<Feed>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        match self.feeds.len() {
            0 => container(text("You don't follow any feeds yet."))
                .padding(20)
                .center_x(Length::Fill)
                .into(),
            _ => Scrollable::new(
                self.feeds
                    .iter()
                    .fold(Column::new().spacing(10), |col, content| {
                        col.push(content.view())
                    })
                    .padding(20)
                    .align_x(Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        }
    }
    pub async fn load_feeds() -> Result<Vec<FeedMeta>, String> {
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
