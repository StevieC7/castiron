use std::collections::HashMap;

use super::feed::{Feed, Message as FeedMessage};
use crate::file_handling::feeds::{delete_associated_episodes_and_xml, load_feeds};
use crate::types::feeds::FeedMeta;
use iced::{
    alignment::Horizontal,
    widget::{column, container, text, Scrollable},
    Element, Length, Task,
};

pub struct FeedList {
    feeds: HashMap<i32, Feed>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedMessage(FeedMessage),
    FeedsLoaded(Result<Vec<FeedMeta>, String>),
    ImageLoaded(i32, Vec<u8>),
    ImageFail,
}

impl FeedList {
    pub fn new() -> Self {
        Self {
            feeds: HashMap::new(),
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FeedMessage(feed_message) => match feed_message {
                FeedMessage::UnfollowFeed(id) => match delete_associated_episodes_and_xml(id) {
                    Ok(_) => Task::perform(load_feeds(), Message::FeedsLoaded),
                    Err(e) => {
                        eprintln!("Error deleting feed: {:?}", e);
                        Task::none()
                    }
                },
                _ => Task::none(),
            },
            Message::FeedsLoaded(list) => match list {
                Err(_) => Task::none(),
                Ok(data) => match data.len() {
                    0 => Task::none(),
                    _ => {
                        let mut hash: HashMap<i32, Feed> = HashMap::new();
                        data.into_iter().for_each(|n| {
                            let Some(feed_title) = &n.feed_title else {
                                todo!()
                            };
                            let feed = Feed::new(n.id, feed_title.to_owned(), None);
                            hash.insert(n.id, feed);
                        });
                        self.feeds = hash;
                        Task::none()
                    }
                },
            },
            Message::ImageLoaded(id, handle) => {
                let feed_to_update = self.feeds.get_mut(&id);
                match feed_to_update {
                    Some(feed) => {
                        let _ = feed.update(FeedMessage::ShowImage(handle));
                    }
                    None => {}
                };
                Task::none()
            }
            // TODO: handle error gracefully
            Message::ImageFail => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        if 0 == self.feeds.len() {
            container(text("You don't follow any feeds yet."))
                .padding(20)
                .center_x(Length::Fill)
                .into()
        } else {
            Scrollable::new(
                column(
                    self.feeds
                        .iter()
                        .map(|(_id, item)| item.view().map(|m| Message::FeedMessage(m))),
                )
                .padding(20)
                .align_x(Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    }
}
