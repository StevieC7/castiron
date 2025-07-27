use super::episode::Episode;
use crate::ui::gui::Message;
use iced::{
    widget::{container, text, Column, Scrollable},
    Element, Length,
};

pub struct EpisodeList {
    pub episodes: Vec<Episode>,
}

impl EpisodeList {
    pub fn new(episodes: Vec<Episode>) -> Self {
        Self { episodes }
    }
    pub fn view(&self) -> Element<Message> {
        match self.episodes.len() {
            0 => container(text("No episodes from feeds you follow."))
                .padding(20)
                .center_x(Length::Fill)
                .into(),
            _ => Scrollable::new(
                self.episodes
                    .iter()
                    .fold(Column::new().spacing(10), |col, content| {
                        col.push(content.view())
                    }),
            )
            .width(Length::Fill)
            .into(),
        }
    }
}
