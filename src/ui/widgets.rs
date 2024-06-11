use iced::alignment::Horizontal;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
// use std::ops::Range;

use crate::file_handling::config::{create_config, read_config};
use crate::file_handling::episodes::{get_episode_by_id, get_episode_list_database};
use crate::file_handling::feeds::get_feed_list_database;
use crate::networking::downloads::{download_episode_by_guid, sync_episode_list};
use crate::types::config::CastironConfig;
use crate::types::episodes::Episode as EpisodeData;
use crate::types::feeds::FeedMeta;

use super::gui::{AppView, Message, PodQueueMessage};
use super::styles::{style_list_item, style_main_area, style_player_area};
use iced::widget::scrollable::Properties;
use iced::widget::{
    button, container, horizontal_space, pick_list, row, text, Column, Row, Scrollable, Text,
};
use iced::widget::{container::Appearance, scrollable::Direction};
use iced::{theme, Alignment, Border, Color, Element, Length, Renderer, Shadow, Theme};
use rodio::{OutputStream, Sink};

pub struct FeedList {
    feeds: Vec<Feed>,
}

impl FeedList {
    pub fn new(feeds: Vec<Feed>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        match self.feeds.len() {
            0 => text("No feeds to show.").into(),
            _ => Scrollable::new(
                container(
                    self.feeds
                        .iter()
                        .fold(Column::new().spacing(10), |col, content| {
                            col.push(content.view())
                        }),
                )
                .align_x(Horizontal::Center),
            )
            .direction(Direction::Both {
                vertical: Properties::default(),
                horizontal: Properties::default(),
            })
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

pub struct Feed {
    id: i32,
    feed_title: String,
}

impl Feed {
    pub fn new(id: i32, feed_title: String) -> Self {
        Self { id, feed_title }
    }
    pub fn view(&self) -> Element<Message> {
        container(row!(
            text(self.feed_title.to_owned()),
            button(text("Unfollow")).on_press(Message::UnfollowFeed(self.id)),
            button(text("View Episodes")).on_press(Message::ViewEpisodesForShow(self.id))
        ))
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            Appearance {
                background: Some(iced::Background::Color(palette.background.strong.color)),
                text_color: None,
                border: Border {
                    color: Color::default(),
                    width: 0.0,
                    radius: [5.0, 5.0, 5.0, 5.0].into(),
                },
                shadow: Shadow::default(),
            }
        })
        .center_x()
        .center_y()
        .padding(20)
        .into()
    }
}

#[derive(Clone)]
pub struct Config {
    pub values: CastironConfig,
    theme: Theme,
}

impl Config {
    pub fn new(values: CastironConfig, theme: Theme) -> Self {
        Self { values, theme }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            row![
                text("Theme"),
                horizontal_space(),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
            ]
            .width(300)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .center_x()
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

pub struct EpisodeList {
    pub episodes: Vec<Episode>,
}

impl EpisodeList {
    pub fn new(episodes: Vec<Episode>) -> Self {
        Self { episodes }
    }
    pub fn view(&self) -> Element<Message> {
        match self.episodes.len() {
            0 => text("No episodes to show.").into(),
            _ => Scrollable::new(
                container(
                    self.episodes
                        .iter()
                        .fold(Column::new().spacing(10), |col, content| {
                            col.push(content.view())
                        }),
                )
                .align_x(Horizontal::Center)
                .style(style_main_area),
            )
            .direction(Direction::Vertical(Properties::default()))
            .width(Length::Fill)
            .height(Length::Fill)
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

pub struct Episode {
    pub id: i32,
    pub feed_id: i32,
    pub guid: String,
    pub title: String,
    pub downloaded: bool,
    pub viewing_from: AppView,
}

impl Episode {
    pub fn new(
        id: i32,
        feed_id: i32,
        guid: String,
        title: String,
        downloaded: bool,
        viewing_from: AppView,
    ) -> Self {
        Self {
            id,
            feed_id,
            guid,
            title,
            downloaded,
            viewing_from,
        }
    }
    pub fn view(&self) -> Element<Message> {
        let action_container: Row<Message, Theme, Renderer> = match self.downloaded {
            true => match self.viewing_from {
                AppView::Queue => {
                    row!(button(text("Play")).on_press(Message::PlayEpisode(self.id)))
                }
                _ => row!(
                    button(text("Delete"))
                        .on_press(Message::DeleteEpisode(self.id))
                        .style(theme::Button::Secondary)
                        .width(Length::FillPortion(3)),
                    horizontal_space().width(Length::FillPortion(1)),
                    button(text("Queue"))
                        .on_press(Message::PodQueueMessage(PodQueueMessage::AddToQueue(
                            self.id
                        )))
                        .style(theme::Button::Secondary)
                        .width(Length::FillPortion(3)),
                    button(text("Play"))
                        .on_press(Message::PlayEpisode(self.id))
                        .width(Length::FillPortion(3))
                ),
            },
            false => row!(button(text("Download")).on_press(Message::DownloadEpisode(self.id)),),
        };
        container(row!(
            text(self.title.to_owned()).width(300),
            action_container
        ))
        .style(style_list_item)
        .height(150)
        .center_x()
        .center_y()
        .padding(20)
        .into()
    }

    pub async fn download_single_episode(id: i32) -> Result<(), String> {
        match download_episode_by_guid(id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from(format!("Error downloading episode: {:?}", e))),
        }
    }
}

#[allow(dead_code)] // The stream isn't called anywhere, but it is necessary to keep the sink alive
pub struct Player {
    pub id: Option<i32>,
    stream: Option<OutputStream>,
    pub sink: Option<Sink>,
}

#[derive(Clone, Debug)]
pub enum PlayerMessage {
    Play,
    Pause,
}

impl Player {
    pub fn new(id: Option<i32>) -> Self {
        match id {
            Some(id) => {
                if let Ok(episode) = get_episode_by_id(id) {
                    if let Ok((stream, stream_handle)) = OutputStream::try_default() {
                        match Sink::try_new(&stream_handle) {
                            Ok(sink) => {
                                println!("episode file name {}", episode.file_name);
                                if let Ok(file) =
                                    File::open(format!("./episodes/{}", episode.file_name))
                                {
                                    let file_buf = BufReader::new(file);
                                    match Decoder::new(file_buf) {
                                        Ok(source) => {
                                            sink.append(source);
                                            sink.play();
                                            Self {
                                                id: Some(episode.id),
                                                stream: Some(stream),
                                                sink: Some(sink),
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("{:?}", e);
                                            Self {
                                                id: None,
                                                stream: None,
                                                sink: None,
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!("failed to open file {}", episode.file_name);
                                    Self {
                                        id: None,
                                        stream: None,
                                        sink: None,
                                    }
                                }
                            }
                            Err(_) => {
                                eprintln!("failed to create Sink");
                                Self {
                                    id: None,
                                    stream: None,
                                    sink: None,
                                }
                            }
                        }
                    } else {
                        eprintln!("failed to create OutputStream");
                        Self {
                            id: None,
                            stream: None,
                            sink: None,
                        }
                    }
                } else {
                    eprintln!("failed to get episode from db");
                    Self {
                        id: None,
                        stream: None,
                        sink: None,
                    }
                }
            }
            None => Self {
                id: None,
                stream: None,
                sink: None,
            },
        }
    }

    pub fn update(&mut self, message: PlayerMessage) {
        match message {
            PlayerMessage::Play => match &self.sink {
                Some(sink) => sink.play(),
                None => (),
            },
            PlayerMessage::Pause => match &self.sink {
                Some(sink) => sink.pause(),
                None => (),
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title: Text<Theme, Renderer> = match self.id {
            Some(id) => {
                let episode = get_episode_by_id(id);
                match episode {
                    Ok(episode) => text(format!("{}", episode.title)),
                    Err(_) => text("Not Playing"),
                }
            }
            None => text("Not Playing"),
        };
        container(
            row!(
                title,
                button(text("Play")).on_press(Message::PlayerMessage(PlayerMessage::Play)),
                button(text("Pause")).on_press(Message::PlayerMessage(PlayerMessage::Pause))
            )
            .spacing(10),
        )
        .width(Length::Fill)
        .padding(20)
        .style(style_player_area)
        .align_x(Horizontal::Center)
        .into()
    }
}
