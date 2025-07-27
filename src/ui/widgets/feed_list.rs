use super::feed::{Feed, Message as FeedMessage};
use crate::file_handling::feeds::{delete_associated_episodes_and_xml, load_feeds};
use crate::types::feeds::FeedMeta;
use iced::widget::{column, container, text};
use iced::{alignment::Horizontal, widget::Scrollable, Element, Length, Task};

pub struct FeedList {
    feeds: Option<Vec<(Feed, Task<FeedMessage>)>>,
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
            Message::FeedMessage(feed_message) => match feed_message {
                FeedMessage::UnfollowFeed(id) => match delete_associated_episodes_and_xml(id) {
                    Ok(_) => Task::perform(load_feeds(), Message::FeedsLoaded),
                    Err(e) => {
                        eprintln!("Error deleting feed: {:?}", e);
                        Task::none()
                    }
                },
                FeedMessage::ViewEpisodesForShow(_id) => Task::none(),
                FeedMessage::ImageState(_state) => Task::none(),
            },
            Message::FeedsLoaded(list) => match list {
                Err(_) => Task::none(),
                Ok(data) => {
                    match data.len() {
                        0 => todo!(),
                        _ => {
                            let list = data
                                .iter()
                                .map(|n| {
                                    let Some(image_path) = &n.image_file_path else {
                                        todo!()
                                    };
                                    let Some(feed_title) = &n.feed_title else {
                                        todo!()
                                    };
                                    // TODO: fix this
                                    Feed::new(
                                        n.id,
                                        feed_title.to_owned(),
                                        Some(image_path.to_owned()),
                                    )
                                })
                                .collect();
                            self.feeds = Some(list);
                        }
                    };
                    Task::none()
                }
            },
        }
    }
    pub fn view(&self) -> Element<Message> {
        match &self.feeds {
            Some(list) => Scrollable::new(
                column(
                    list.iter()
                        .map(|(item, _task)| item.view().map(|m| Message::FeedMessage(m))),
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
