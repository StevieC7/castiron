use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
// use std::ops::Range;

use crate::file_handling::config::{create_config, read_config};
use crate::file_handling::episodes::{get_episode_by_guid, get_episode_list_database};
use crate::file_handling::feeds::get_feed_list_database;
use crate::networking::downloads::{download_episode_by_guid, sync_episode_list};
use crate::types::config::CastironConfig;
use crate::types::episodes::Episode as EpisodeData;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use iced::widget::scrollable::Properties;
use iced::widget::{
    button, column, container, row, text, Button, Column, Scrollable, Text, Toggler,
};
use iced::widget::{container::Appearance, scrollable::Direction};
use iced::{Border, Color, Element, Length, Renderer, Shadow, Theme};
use rodio::{OutputStream, Sink};

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

pub struct Feed {
    feed_title: String,
}

impl Feed {
    pub fn new(feed_title: String) -> Self {
        Self { feed_title }
    }
    pub fn view(&self) -> Element<Message> {
        container(row!(text(self.feed_title.to_owned())))
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

pub struct EpisodeList {
    episodes: Vec<Episode>,
    // visible_episodes: Range<usize>,
    // previous_offset: f32,
}

// #[derive(Clone, Debug)]
// pub enum EpisodesMessage {
//     Scrolled(f32, usize, usize),
// }

impl EpisodeList {
    pub fn new(episodes: Vec<Episode>) -> Self {
        Self {
            episodes,
            // visible_episodes: Range { start: 0, end: 20 },
            // previous_offset: 0.0,
        }
    }
    pub fn view(&self) -> Element<Message> {
        // if let Some(visible) = self.episodes.get(self.visible_episodes.to_owned()) {
        Scrollable::new(column![
            // visible
            self.episodes
                .iter()
                .fold(Column::new().spacing(10), |col, content| {
                    col.push(content.view())
                })
        ])
        // .on_scroll(|viewport| {
        //     fn float_to_usize_or_zero(float: f32) -> usize {
        //         let float_stringified = float.to_string();
        //         match float_stringified.split('.').take(1).next() {
        //             Some(string) => match string.parse::<usize>() {
        //                 Ok(num) => num,
        //                 Err(_) => 0,
        //             },
        //             None => 0,
        //         }
        //     }
        //     // TODO: create state to hold how many episodes should be included in Range, and dynamically adjust this based on window resizes
        //     let episode_height: usize = 250 + 10; // use episode height plus whatever spacing is added above
        //     let episode_range_length: usize =
        //         (float_to_usize_or_zero(viewport.bounds().height) / episode_height) + 10; // Always have more than needed so this is smooth
        //     let mut start = self.visible_episodes.start;
        //     let offset = float_to_usize_or_zero(viewport.absolute_offset().y);
        //     let episodes_to_move = offset / episode_height;
        //     match viewport.absolute_offset().y - self.previous_offset < 0.0 {
        //         true => match start.checked_sub(episodes_to_move) {
        //             Some(num) => {
        //                 start = num;
        //             }
        //             None => {
        //                 start = 0;
        //             }
        //         },
        //         false => match start + episode_range_length + episodes_to_move
        //             > self.episodes.len()
        //         {
        //             true => {
        //                 println!("End would have been greater than episodes length");
        //                 start = self.episodes.len() - episode_range_length - 10 + 1;
        //             }
        //             false => {
        //                 start = start + episodes_to_move;
        //             }
        //         },
        //     };
        //     println!(
        //         "start: {}, end: {}, episodes_to_move: {}",
        //         start,
        //         start + episode_range_length,
        //         episodes_to_move
        //     );
        //     Message::EpisodesMessage(EpisodesMessage::Scrolled(
        //         viewport.absolute_offset().y,
        //         start,
        //         start + episode_range_length,
        //     ))
        // })
        .direction(Direction::Vertical(Properties::default()))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
        // } else {
        //     text("Oops").into()
        // }
    }
    // pub fn update(&mut self, message: EpisodesMessage) {
    //     match message {
    //         EpisodesMessage::Scrolled(prev_offset, start, end) => {
    //             self.visible_episodes = Range { start, end };
    //             self.previous_offset = prev_offset;
    //         }
    //     }
    // }
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
    guid: String,
    title: String,
    downloaded: bool,
}

impl Episode {
    pub fn new(guid: String, title: String, downloaded: bool) -> Self {
        Self {
            guid,
            title,
            downloaded,
        }
    }
    pub fn view(&self) -> Element<Message> {
        let action_button: Button<Message, Theme, Renderer> = match self.downloaded {
            true => button(text("Play")).on_press(Message::PlayEpisode(self.guid.to_owned())),
            false => {
                button(text("Download")).on_press(Message::DownloadEpisode(self.guid.to_owned()))
            }
        };
        container(row!(text(self.title.to_owned()), action_button))
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
            .height(250)
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

#[allow(dead_code)] // The stream isn't called anywhere, but it is necessary to keep the sink alive
pub struct Player {
    guid: Option<String>,
    stream: Option<OutputStream>,
    sink: Option<Sink>,
}

#[derive(Clone, Debug)]
pub enum PlayerMessage {
    Play,
    Pause,
}

impl Player {
    pub fn new(guid: Option<String>) -> Self {
        match guid {
            Some(guid) => {
                if let Ok(episode) = get_episode_by_guid(&guid) {
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
                                                guid: Some(episode.guid),
                                                stream: Some(stream),
                                                sink: Some(sink),
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("{:?}", e);
                                            Self {
                                                guid: None,
                                                stream: None,
                                                sink: None,
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!("failed to open file {}", episode.file_name);
                                    Self {
                                        guid: None,
                                        stream: None,
                                        sink: None,
                                    }
                                }
                            }
                            Err(_) => {
                                eprintln!("failed to create Sink");
                                Self {
                                    guid: None,
                                    stream: None,
                                    sink: None,
                                }
                            }
                        }
                    } else {
                        eprintln!("failed to create OutputStream");
                        Self {
                            guid: None,
                            stream: None,
                            sink: None,
                        }
                    }
                } else {
                    eprintln!("failed to get episode from db");
                    Self {
                        guid: None,
                        stream: None,
                        sink: None,
                    }
                }
            }
            None => Self {
                guid: None,
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
        let title: Text<Theme, Renderer> = match &self.guid {
            Some(guid) => {
                let episode = get_episode_by_guid(&guid);
                match episode {
                    Ok(episode) => text(format!("{}", episode.title)),
                    Err(_) => text("Not Playing"),
                }
            }
            None => text("Not Playing"),
        };
        container(column!(
            title,
            row!(
                button(text("Play")).on_press(Message::PlayerMessage(PlayerMessage::Play)),
                button(text("Pause")).on_press(Message::PlayerMessage(PlayerMessage::Pause))
            )
        ))
        .into()
    }
}
