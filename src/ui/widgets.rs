pub mod episode;
pub mod feed;

use iced::alignment::Horizontal;
use iced::time;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use crate::file_handling::episodes::{get_episode_by_id, get_episode_list_database};
use crate::file_handling::feeds::get_feed_list_database;
use crate::networking::feeds::sync_episode_list;
use crate::types::config::CastironConfig;
use crate::types::episodes::Episode as EpisodeData;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use episode::Episode;
use feed::Feed;

use iced::widget::{
    button, container, horizontal_space, pick_list, progress_bar, row, text, Column, Scrollable,
    Text,
};
use iced::{Alignment, Element, Length, Renderer, Subscription, Theme};
use rodio::{OutputStream, OutputStreamBuilder, Sink};

pub struct FeedList {
    feeds: Vec<Feed>,
}

impl FeedList {
    pub fn new(feeds: Vec<Feed>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        match self.feeds.len() {
            0 => container(text("You don't follow any feeds yet."))
                .padding(20)
                .center_x(Length::Fill)
                .into(),
            _ => Scrollable::new(
                self.feeds
                    .iter()
                    .fold(Column::new().spacing(10), |col, content| {
                        col.push(content.view())
                    })
                    .padding(20)
                    .align_x(Horizontal::Center),
            )
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

#[derive(Clone)]
pub struct Config {
    pub values: CastironConfig,
    pub theme: Theme,
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
            .padding(20)
            .align_y(Alignment::Center),
        )
        .center_x(Length::Fill)
        .into()
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

// pub struct Episode {
//     pub id: i32,
//     pub feed_id: i32,
//     pub guid: String,
//     pub title: String,
//     pub downloaded: bool,
//     pub viewing_from: AppView,
//     pub image_handle: Option<image::Handle>,
// }

// impl Episode {
//     pub fn new(
//         id: i32,
//         feed_id: i32,
//         guid: String,
//         title: String,
//         downloaded: bool,
//         viewing_from: AppView,
//         image_handle: Option<image::Handle>,
//     ) -> Self {
//         Self {
//             id,
//             feed_id,
//             guid,
//             title,
//             downloaded,
//             viewing_from,
//             image_handle,
//         }
//     }
//     pub fn view(&self) -> Element<Message> {
//         let action_container: Row<Message, Theme, Renderer> = match self.downloaded {
//             true => match self.viewing_from {
//                 AppView::Queue => {
//                     row!(button(text("Play")).on_press(Message::PlayEpisode(self.id)))
//                 }
//                 _ => row!(
//                     button(text("Delete"))
//                         .on_press(Message::DeleteEpisode(self.id))
//                         .width(Length::FillPortion(3)),
//                     horizontal_space().width(Length::FillPortion(1)),
//                     button(text("Queue"))
//                         .on_press(Message::PodQueueMessage(PodQueueMessage::AddToQueue(
//                             self.id
//                         )))
//                         .width(Length::FillPortion(3)),
//                     button(text("Play"))
//                         .on_press(Message::PlayEpisode(self.id))
//                         .width(Length::FillPortion(3))
//                 ),
//             },
//             false => row!(button(text("Download")).on_press(Message::DownloadEpisode(self.id)),),
//         };
//         container(row!(
//             match &self.image_handle {
//                 Some(handle) => image(handle).height(100),
//                 None => image(""),
//             },
//             text(self.title.to_owned()).width(300),
//             action_container
//         ))
//         .width(Length::Shrink)
//         .max_width(600)
//         .padding(20)
//         .center_y(Length::Shrink)
//         .into()
//     }

//     pub async fn download_single_episode(id: i32) -> Result<(), String> {
//         match download_episode_by_guid(id).await {
//             Ok(_) => Ok(()),
//             Err(e) => Err(String::from(format!("Error downloading episode: {:?}", e))),
//         }
//     }
// }

#[allow(dead_code)] // Sink is the handle to the stream, but if stream is dropped, playback stops.
#[derive(Default)]
pub struct Player {
    pub id: Option<i32>,
    stream: Option<OutputStream>,
    pub sink: Option<Sink>,
    pub progress: f32,
    pub duration_seconds: f32,
}

#[derive(Clone, Debug)]
pub enum PlayerMessage {
    Play,
    Pause,
    Progress,
}

impl Player {
    pub fn new(id: Option<i32>) -> Self {
        match id {
            None => Self::default(),
            Some(id) => match get_episode_by_id(id) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    Self::default()
                }
                Ok(episode) => match OutputStreamBuilder::open_default_stream() {
                    Err(e) => {
                        eprintln!("{:?}", e);
                        Self::default()
                    }
                    Ok(stream_handle) => {
                        let sink = Sink::connect_new(&stream_handle.mixer());
                        match File::open(format!("./episodes/{}", episode.file_name)) {
                            Err(e) => {
                                eprintln!("{:?}", e);
                                Self::default()
                            }
                            Ok(file) => {
                                let file_meta = file.metadata();
                                let byte_len = match file_meta {
                                    Ok(meta) => meta.len(),
                                    Err(_) => 0,
                                };
                                let file_buf = BufReader::new(file);
                                match Decoder::builder()
                                    .with_data(file_buf)
                                    .with_byte_len(byte_len)
                                    .build()
                                {
                                    Err(e) => {
                                        eprintln!("{:?}", e);
                                        Self::default()
                                    }
                                    Ok(source) => {
                                        let duration_secs = match source.total_duration() {
                                            Some(dur) => dur.as_secs_f32(),
                                            None => 0.0,
                                        };
                                        sink.append(source);
                                        sink.play();
                                        Self {
                                            id: Some(episode.id),
                                            stream: Some(stream_handle),
                                            sink: Some(sink),
                                            progress: 0.0,
                                            duration_seconds: duration_secs,
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
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
            PlayerMessage::Progress => match &self.sink {
                Some(sink) => match sink.empty() {
                    true => {
                        self.progress = 0.0;
                    }
                    false => {
                        let position = sink.get_pos();
                        self.progress = position.as_secs_f32();
                    }
                },
                None => (),
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![time::every(Duration::from_millis(100))
            .map(|_| Message::PlayerMessage(PlayerMessage::Progress))])
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
                button(text("Pause")).on_press(Message::PlayerMessage(PlayerMessage::Pause)),
                progress_bar(0.0..=self.duration_seconds, self.progress)
            )
            .spacing(10),
        )
        .width(Length::Fill)
        .padding(20)
        .align_x(Horizontal::Center)
        .into()
    }
}
