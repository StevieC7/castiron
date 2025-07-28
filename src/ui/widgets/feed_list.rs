use std::collections::HashMap;

use super::feed::{Feed, Message as FeedMessage};
use crate::file_handling::feeds::{delete_associated_episodes_and_xml, get_feed_by_id, load_feeds};
use crate::types::feeds::FeedMeta;
use iced::{
    advanced::image::Handle,
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
    ImageLoaded(i32, Handle),
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
                FeedMessage::ViewEpisodesForShow(_id) => Task::none(),
            },
            Message::FeedsLoaded(list) => match list {
                Err(_) => Task::none(),
                Ok(data) => match data.len() {
                    0 => Task::none(),
                    _ => {
                        let mut hash: HashMap<i32, Feed> = HashMap::new();
                        let mut task_batch: Vec<Task<Message>> = Vec::new();
                        data.into_iter().for_each(|n| {
                            let Some(feed_title) = &n.feed_title else {
                                todo!()
                            };
                            let feed = Feed::new(n.id, feed_title.to_owned(), None);
                            hash.insert(n.id, feed);
                            task_batch.push(Task::perform(
                                async move {
                                    (
                                        n.id,
                                        Handle::from_path(
                                            n.image_file_path
                                                .to_owned()
                                                .unwrap_or(String::from("")),
                                        ),
                                    )
                                },
                                |(id, handle)| Message::ImageLoaded(id, handle),
                            ));
                        });
                        self.feeds = hash;
                        // NOW: fix this
                        Task::batch(task_batch)
                    }
                },
            },
            Message::ImageLoaded(id, handle) => {
                // TODO: initiate rebuild of affected feeds
                let new_feed = get_feed_by_id(id);
                match new_feed {
                    Ok(meta) => {
                        let replace_with = Feed::new(
                            meta.id,
                            meta.feed_title.unwrap_or(String::from("")),
                            Some(handle),
                        );
                        self.feeds.insert(id, replace_with);
                        Task::none()
                    }
                    Err(_) => Task::none(),
                }
            }
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
