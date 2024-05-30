use crate::file_handling::config::{create_config, read_config};
use crate::file_handling::feeds::get_feed_list_database;
use crate::networking::downloads::sync_episode_list;
use crate::types::config::CastironConfig;
use crate::types::episodes::Episode as EpisodeData;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use iced::widget::container::Appearance;
use iced::widget::{container, row, text, Column, Toggler};
use iced::{Border, Color, Element, Shadow};

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

    pub async fn fetch_config() -> Result<CastironConfig, String> {
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
pub struct Episodes {
    episodes: Vec<Episode>,
}

impl Episodes {
    pub fn new(episodes: Vec<Episode>) -> Self {
        Self { episodes }
    }
    pub fn view(&self) -> Element<Message> {
        let col = Column::new();
        let episodes: Element<Message> = self
            .episodes
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(content.view())
            })
            .into();
        col.push(episodes).into()
    }
    pub async fn fetch_episodes() -> Result<Option<Vec<EpisodeData>>, String> {
        let result = sync_episode_list().await;
        match result {
            Ok(res) => match res {
                Some(val) => Ok(Some(val)),
                None => Ok(None),
            },
            Err(e) => Err(String::from(format!(
                "Error fetching episodes from database: {:?}",
                e
            ))),
        }
    }
}

#[derive(Clone)]
pub struct Episode {
    title: String,
    file_path: Option<String>,
}

impl Episode {
    pub fn new(episode: EpisodeData) -> Self {
        Self {
            title: episode.title,
            file_path: episode.file_path,
        }
    }
    pub fn view(&self) -> Element<Message> {
        container(row!(text(self.title.to_owned())))
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
