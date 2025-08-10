use std::collections::HashMap;

use super::episode::Episode;
use crate::{types::episodes::EpisodeMeta, ui::widgets::episode::Message as EpisodeMessage};
use iced::{
    alignment::Horizontal,
    widget::{column, container, text, Scrollable},
    Element, Length, Task,
};

pub struct EpisodeList {
    pub episodes: HashMap<i32, Episode>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EpisodeMessage((i32, EpisodeMessage)),
    EpisodesLoaded(Result<Vec<EpisodeMeta>, String>),
}

impl EpisodeList {
    pub fn new() -> Self {
        Self {
            episodes: HashMap::new(),
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EpisodeMessage((id, episode_message)) => {
                let episode_to_update = self.episodes.get_mut(&id);
                match episode_to_update {
                    Some(episode) => {
                        let task = episode
                            .update(episode_message)
                            .map(move |msg| Message::EpisodeMessage((id, msg)));
                        task
                    }
                    None => Task::none(),
                }
            }
            Message::EpisodesLoaded(episodes_meta) => match episodes_meta {
                Err(_) => Task::none(),
                Ok(data) => match data.len() {
                    0 => Task::none(),
                    _ => {
                        let mut hash: HashMap<i32, Episode> = HashMap::new();
                        data.into_iter().for_each(|n| {
                            let episode =
                                Episode::new(n.id, n.feed_id, n.guid, n.title, n.downloaded, None);
                            hash.insert(n.id, episode);
                        });
                        self.episodes = hash;
                        Task::none()
                    }
                },
            },
        }
    }
    pub fn view(&self) -> Element<Message> {
        match self.episodes.len() {
            0 => container(text("No episodes from feeds you follow."))
                .padding(20)
                .center_x(Length::Fill)
                .into(),
            _ => Scrollable::new(
                column(self.episodes.iter().map(|(id, item)| {
                    item.view()
                        .map(|m| Message::EpisodeMessage((id.clone(), m)))
                }))
                .padding(20)
                .align_x(Horizontal::Center),
            )
            .width(Length::Fill)
            .into(),
        }
    }
}
