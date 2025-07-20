use super::episode::Episode;
use crate::{
    file_handling::episodes::get_episode_list_database, networking::feeds::sync_episode_list,
    types::episodes::Episode as EpisodeData, ui::gui::Message,
};
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

    pub async fn load_episodes() -> Result<Option<Vec<EpisodeData>>, String> {
        match get_episode_list_database() {
            Ok(data) => Ok(Some(data)),
            Err(e) => Err(String::from(format!(
                "Error fetching episodes from database: {:?}",
                e
            ))),
        }
    }

    pub async fn sync_episodes() -> Result<Option<Vec<EpisodeData>>, String> {
        let result = sync_episode_list().await;
        match result {
            Ok(res) => match res {
                Some(val) => Ok(Some(val)),
                None => Ok(None),
            },
            Err(e) => Err(String::from(format!("Error syncing episodes: {:?}", e))),
        }
    }
}
