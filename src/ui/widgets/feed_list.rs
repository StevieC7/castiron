use super::feed::{Feed, Message as FeedMessage};
use crate::file_handling::feeds::{delete_associated_episodes_and_xml, load_feeds};
use crate::types::feeds::FeedMeta;
use iced::widget::{column, container, text};
use iced::{alignment::Horizontal, widget::Scrollable, Element, Length, Task};

pub struct FeedList {
    feeds: Option<Vec<Feed>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedMessage(FeedMessage),
    FeedsLoaded(Result<Vec<FeedMeta>, String>),
}

impl FeedList {
    pub fn new() -> Self {
        Self { feeds: None }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FeedMessage(feed_message) => {
                if let Some(feeds) = &mut self.feeds {
                    let mut tasks: Vec<Task<Message>> = feeds
                        .iter_mut()
                        .map(|feed| feed.update(feed_message.clone()).map(Message::FeedMessage))
                        .collect();
                    if let FeedMessage::UnfollowFeed(id) = feed_message {
                        let load_feed_task = match delete_associated_episodes_and_xml(id) {
                            Ok(_) => Task::perform(load_feeds(), Message::FeedsLoaded),
                            Err(e) => {
                                eprintln!("Error deleting feed: {:?}", e);
                                Task::none()
                            }
                        };
                        tasks.push(load_feed_task);
                    }
                    Task::batch(tasks)
                } else {
                    Task::none()
                }
            }
            Message::FeedsLoaded(list) => match list {
                Err(_) => Task::none(),
                Ok(data) => match data.len() {
                    0 => Task::none(),
                    _ => {
                        let mut tasks: Vec<Task<Message>> = Vec::new();
                        let list = data
                            .iter()
                            .map(|n| {
                                let Some(image_path) = &n.image_file_path else {
                                    todo!()
                                };
                                let Some(feed_title) = &n.feed_title else {
                                    todo!()
                                };
                                let (element, task) = Feed::new(
                                    n.id,
                                    feed_title.to_owned(),
                                    Some(image_path.to_owned()),
                                );
                                tasks.push(task.map(Message::FeedMessage));
                                element
                            })
                            .collect();
                        self.feeds = Some(list);
                        Task::batch(tasks)
                    }
                },
            },
        }
    }
    pub fn view(&self) -> Element<Message> {
        match &self.feeds {
            Some(list) => Scrollable::new(
                column(
                    list.iter()
                        .map(|item| item.view().map(|m| Message::FeedMessage(m))),
                )
                .padding(20)
                .align_x(Horizontal::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            None => container(text("You don't follow any feeds yet."))
                .padding(20)
                .center_x(Length::Fill)
                .into(),
        }
    }
}
