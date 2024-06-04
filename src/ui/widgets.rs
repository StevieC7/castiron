use std::path::Path;

use crate::file_handling::config::{create_config, read_config};
use crate::file_handling::episodes::get_episode_list_database;
use crate::file_handling::feeds::get_feed_list_database;
use crate::networking::downloads::{download_episode_by_guid, sync_episode_list};
use crate::types::config::CastironConfig;
use crate::types::episodes::Episode as EpisodeData;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use iced::widget::scrollable::Properties;
use iced::widget::{button, column, container, row, text, Button, Column, Scrollable, Toggler};
use iced::widget::{container::Appearance, scrollable::Direction};
use iced::{Border, Color, Element, Renderer, Shadow, Theme};

#[derive(Clone)]
pub struct FeedList {
    feeds: Vec<Feed>,
}

impl FeedList {
    pub fn new(feeds: Vec<Feed>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        Scrollable::new(column![self
            .feeds
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(content.view())
            })])
        .direction(Direction::Both {
            vertical: Properties::default(),
            horizontal: Properties::default(),
        })
        .into()
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

#[derive(Clone)]
pub struct Feed {
    feed_url: String,
}

impl Feed {
    pub fn new(feed_url: String) -> Self {
        Self { feed_url }
    }
    pub fn view(&self) -> Element<Message> {
        container(row!(text(self.feed_url.to_owned())))
            .style(Appearance {
                background: Some(iced::Background::Color(Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                })),
                text_color: None,
                border: Border {
                    color: Color::default(),
                    width: 0.0,
                    radius: [5.0, 5.0, 5.0, 5.0].into(),
                },
                shadow: Shadow::default(),
            })
            .max_width(500)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

#[derive(Clone)]
pub struct Config {
    values: CastironConfig,
}

impl Config {
    pub fn new(values: CastironConfig) -> Self {
        Self { values }
    }

    pub fn view(&self) -> Element<Message> {
        let vals = self.values.to_owned();
        let column = Column::new();
        column
            .push(Toggler::new(
                String::from("Automatically download new episodes?"),
                vals.auto_dl_new,
                move |n| {
                    Message::SaveConfig(Some(CastironConfig {
                        auto_dl_new: n,
                        auto_rm_after_listen: vals.auto_rm_after_listen,
                        dark_mode: vals.dark_mode,
                    }))
                },
            ))
            .into()
    }

    pub async fn load_config() -> Result<CastironConfig, String> {
        let result = read_config();
        match result {
            Ok(conf) => Ok(conf),
            Err(_) => {
                let create_result = create_config(None);
                match create_result {
                    Ok(new_conf) => Ok(new_conf),
                    Err(e) => Err(String::from(format!("Error fetching config: {:?}", e))),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct EpisodeList {
    episodes: Vec<Episode>,
}

impl EpisodeList {
    pub fn new(episodes: Vec<Episode>) -> Self {
        Self { episodes }
    }
    pub fn view(&self) -> Element<Message> {
        Scrollable::new(column![self
            .episodes
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(content.view())
            })])
        .direction(Direction::Vertical(Properties::default()))
        .into()
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

#[derive(Clone)]
pub struct Episode {
    guid: String,
    title: String,
    file_name: String,
}

impl Episode {
    pub fn new(guid: String, title: String, file_name: String) -> Self {
        Self {
            guid,
            title,
            file_name,
        }
    }
    pub fn view(&self) -> Element<Message> {
        let action_button: Button<Message, Theme, Renderer> =
            match Path::new(format!("./episodes/{}", self.file_name).as_str()).exists() {
                true => button(text("Play")).on_press(Message::PlayEpisode),
                false => button(text("Download"))
                    .on_press(Message::DownloadEpisode(self.guid.to_owned())),
            };
        container(row!(text(self.title.to_owned()), action_button))
            .style(Appearance {
                background: Some(iced::Background::Color(Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                })),
                text_color: None,
                border: Border {
                    color: Color::default(),
                    width: 0.0,
                    radius: [5.0, 5.0, 5.0, 5.0].into(),
                },
                shadow: Shadow::default(),
            })
            .max_width(500)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    pub async fn download_single_episode(guid: String) -> Result<(), String> {
        match download_episode_by_guid(guid).await {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from(format!("Error downloading episode: {:?}", e))),
        }
    }
}
