use std::time::Duration;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{
    button, column, container, row, text, text_input, vertical_space, Column, Rule, Scrollable,
};
use iced::{executor, theme, Alignment, Application, Command, Element, Length};
use iced::{time, Subscription, Theme};

use crate::file_handling::config::{convert_theme_string_to_enum, create_config};
use crate::file_handling::episodes::{
    delete_episode_from_fs, get_episode_by_id, get_episodes_by_feed_id,
};
use crate::file_handling::feeds::{
    add_feed_to_database, delete_associated_episodes_and_xml, get_feed_by_id,
};
use crate::file_handling::setup::{
    create_episodes_directory_if_not_existing, create_shows_directory_if_not_existing,
};
use crate::types::config::CastironConfig;
use crate::types::{episodes::Episode as EpisodeData, feeds::FeedMeta};

use super::styles::{style_main_area, style_sidebar, style_sidebar_rule, MyButtonStyle};
use super::widgets::{Config, Episode, EpisodeList, Feed, FeedList, Player, PlayerMessage};

pub struct AppLayout {
    app_view: AppView,
    feeds: FeedList,
    episodes: EpisodeList,
    episodes_for_show: EpisodeList,
    castiron_config: Option<Config>,
    feed_to_add: String,
    player: Player,
    queue: Vec<Episode>,
    theme: Theme,
}

pub enum AppView {
    Feeds,
    Episodes,
    EpisodesForShow(i32),
    Config,
    Queue,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewEpisodes,
    ViewFeeds,
    ViewEpisodesForShow(i32),
    ViewQueue,
    ViewConfig,
    AddFeed,
    UnfollowFeed(i32),
    SyncEpisodes,
    DownloadEpisode(i32),
    PlayEpisode(i32),
    DeleteEpisode(i32),
    SetupCompleted(Result<(), String>),
    ConfigLoaded(Result<CastironConfig, String>),
    FeedsLoaded(Result<Vec<FeedMeta>, String>),
    EpisodesLoaded(Result<Option<Vec<EpisodeData>>, String>),
    EpisodesSynced(Result<Option<Vec<EpisodeData>>, String>),
    EpisodeDownloaded(Result<(), String>),
    FeedToAddUpdated(String),
    PlayerProgressed,
    PlayerMessage(PlayerMessage),
    PodQueueMessage(PodQueueMessage),
    ThemeChanged(Theme),
}

#[derive(Debug, Clone)]
pub enum PodQueueMessage {
    RemoveFromQueue(i32),
    AddToQueue(i32),
    MoveToPosition(usize, usize),
}

impl AppLayout {
    pub fn update_queue(&mut self) {
        let new_queue: Vec<Episode> = self
            .queue
            .iter()
            .map(|episode| {
                let updated_episode = get_episode_by_id(episode.id);
                match updated_episode {
                    Ok(u_episode) => Episode::new(
                        u_episode.id,
                        u_episode.feed_id,
                        u_episode.guid,
                        u_episode.title,
                        u_episode.downloaded,
                        AppView::Queue,
                    ),
                    Err(_) => Episode::new(
                        episode.id,
                        episode.feed_id,
                        episode.guid.to_owned(),
                        episode.title.to_owned(),
                        episode.downloaded,
                        AppView::Queue,
                    ),
                }
            })
            .collect();
        self.queue = new_queue;
    }
    pub fn view_queue(&self) -> Element<Message> {
        let mut col_len: usize = 0;
        let column = self
            .queue
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col_len = col_len + 1;
                col.push(
                    row![
                        content.view(),
                        container(column![
                            button(text("Move Up"))
                                .on_press(Message::PodQueueMessage(
                                    PodQueueMessage::MoveToPosition(
                                        col_len.wrapping_sub(1),
                                        col_len.wrapping_sub(2)
                                    )
                                ))
                                .width(100)
                                .style(theme::Button::Secondary),
                            button(text("X"))
                                .on_press(Message::PodQueueMessage(
                                    PodQueueMessage::RemoveFromQueue(content.id)
                                ))
                                .width(100)
                                .style(theme::Button::Secondary),
                            button(text("Move Down"))
                                .on_press(Message::PodQueueMessage(
                                    PodQueueMessage::MoveToPosition(
                                        col_len.wrapping_sub(1),
                                        col_len
                                    )
                                ))
                                .width(100)
                                .style(theme::Button::Secondary)
                        ])
                        .height(Length::Fill)
                        .align_y(Vertical::Center)
                    ]
                    .height(150),
                )
            });
        Scrollable::new(container(column).align_x(Horizontal::Center))
            .direction(Direction::Vertical(Properties::default()))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Application for AppLayout {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                app_view: AppView::Feeds,
                feeds: FeedList::new(Vec::new()),
                episodes: EpisodeList::new(Vec::new()),
                episodes_for_show: EpisodeList::new(Vec::new()),
                castiron_config: None,
                feed_to_add: String::new(),
                player: Player::new(None),
                queue: Vec::new(),
                theme: Theme::default(),
            },
            Command::batch([
                Command::perform(
                    create_shows_directory_if_not_existing(),
                    Message::SetupCompleted,
                ),
                Command::perform(
                    create_episodes_directory_if_not_existing(),
                    Message::SetupCompleted,
                ),
                Command::perform(Config::load_config(), Message::ConfigLoaded),
                Command::perform(FeedList::load_feeds(), Message::FeedsLoaded),
                Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Castiron")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FeedsLoaded(feeds) => match feeds {
                Err(_) => Command::none(),
                Ok(data) => {
                    let feed_list = data
                        .iter()
                        .map(|n| match &n.feed_title {
                            Some(feed_title) => Feed::new(n.id, feed_title.to_owned()),
                            None => Feed::new(n.id, n.feed_url.to_owned()),
                        })
                        .collect();
                    self.feeds = FeedList::new(feed_list);
                    Command::none()
                }
            },
            Message::EpisodesLoaded(episodes) => match episodes {
                Err(e) => {
                    println!("Episode loading failed: {:?}", e);
                    Command::none()
                }
                Ok(data) => {
                    match data {
                        Some(found) => {
                            let episode_list = found
                                .iter()
                                .map(|n| {
                                    Episode::new(
                                        n.id,
                                        n.feed_id,
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                        AppView::Episodes,
                                    )
                                })
                                .collect();
                            self.episodes = EpisodeList::new(episode_list);
                            match self.app_view {
                                AppView::EpisodesForShow(id) => {
                                    self.episodes_for_show = EpisodeList::new(
                                        found
                                            .iter()
                                            .filter(|ep| ep.feed_id == id)
                                            .map(|n| {
                                                Episode::new(
                                                    n.id,
                                                    n.feed_id,
                                                    n.guid.to_owned(),
                                                    n.title.to_owned(),
                                                    n.downloaded,
                                                    AppView::EpisodesForShow(id),
                                                )
                                            })
                                            .collect(),
                                    );
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    };
                    Command::none()
                }
            },
            Message::EpisodesSynced(episodes) => match episodes {
                Err(e) => {
                    println!("Episode sync failed: {:?}", e);
                    Command::none()
                }
                Ok(data) => {
                    match data {
                        Some(found) => {
                            let episode_list = found
                                .iter()
                                .map(|n| {
                                    Episode::new(
                                        n.id,
                                        n.feed_id,
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                        AppView::Episodes,
                                    )
                                })
                                .collect();
                            self.episodes = EpisodeList::new(episode_list);
                        }
                        None => {}
                    };
                    Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded)
                }
            },
            Message::ConfigLoaded(config) => match config {
                Err(_) => Command::none(),
                Ok(data) => {
                    self.theme = convert_theme_string_to_enum(data.theme.clone());
                    self.castiron_config = Some(Config::new(data, self.theme.clone()));
                    Command::none()
                }
            },
            Message::ViewEpisodes => {
                self.app_view = AppView::Episodes;
                Command::none()
            }
            Message::ViewFeeds => {
                self.app_view = AppView::Feeds;
                Command::none()
            }
            Message::ViewEpisodesForShow(id) => {
                let episodes_for_show_result = get_episodes_by_feed_id(id);
                match episodes_for_show_result {
                    Ok(episodes_for_show) => {
                        let episode_list = episodes_for_show
                            .iter()
                            .map(|n| {
                                Episode::new(
                                    n.id,
                                    n.feed_id,
                                    n.guid.to_owned(),
                                    n.title.to_owned(),
                                    n.downloaded,
                                    AppView::EpisodesForShow(id),
                                )
                            })
                            .collect();
                        self.episodes_for_show = EpisodeList::new(episode_list);
                    }
                    Err(_) => {}
                }
                self.app_view = AppView::EpisodesForShow(id);
                Command::none()
            }
            Message::ViewQueue => {
                self.app_view = AppView::Queue;
                Command::none()
            }
            Message::ViewConfig => {
                self.app_view = AppView::Config;
                Command::none()
            }
            Message::AddFeed => {
                let result = add_feed_to_database(self.feed_to_add.to_owned());
                self.feed_to_add = String::new();
                match result {
                    Ok(_) => Command::perform(FeedList::load_feeds(), Message::FeedsLoaded),
                    Err(_) => Command::none(),
                }
            }
            Message::FeedToAddUpdated(val) => {
                self.feed_to_add = val;
                Command::none()
            }
            Message::SyncEpisodes => {
                Command::perform(EpisodeList::sync_episodes(), Message::EpisodesSynced)
            }
            Message::DownloadEpisode(guid) => Command::perform(
                Episode::download_single_episode(guid),
                Message::EpisodeDownloaded,
            ),
            Message::EpisodeDownloaded(result) => match result {
                Ok(_) => {
                    self.update_queue();
                    Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded)
                }
                Err(e) => {
                    println!("Error downloading episode: {e}");
                    Command::none()
                }
            },
            Message::PlayEpisode(id) => {
                self.player = Player::new(Some(id));
                // TODO: handle checking for episode in queue and, if found, removing it from queue
                let found_idx = self.queue.iter().position(|episode| episode.id == id);
                match found_idx {
                    Some(idx) => {
                        self.queue.remove(idx);
                    }
                    None => {}
                }
                Command::none()
            }
            Message::PlayerMessage(message) => {
                self.player.update(message);
                Command::none()
            }
            Message::DeleteEpisode(guid) => match delete_episode_from_fs(guid) {
                Ok(_) => {
                    self.update_queue();
                    Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded)
                }
                Err(e) => {
                    eprintln!("Error deleting episode: {:?}", e);
                    Command::none()
                }
            },
            Message::UnfollowFeed(id) => match delete_associated_episodes_and_xml(id) {
                Ok(_) => Command::batch([
                    Command::perform(FeedList::load_feeds(), Message::FeedsLoaded),
                    Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
                ]),
                Err(e) => {
                    eprintln!("Error deleting feed: {:?}", e);
                    Command::none()
                }
            },
            Message::PodQueueMessage(pod_queue_message) => {
                match pod_queue_message {
                    PodQueueMessage::RemoveFromQueue(id) => {
                        let position = self.queue.iter().position(|pod| pod.id == id);
                        match position {
                            Some(index) => {
                                println!("index {} removed", index);
                                self.queue.remove(index);
                            }
                            None => {}
                        }
                    }
                    PodQueueMessage::AddToQueue(id) => {
                        let episode = get_episode_by_id(id);
                        match episode {
                            Ok(ep) => self.queue.push(Episode::new(
                                ep.id,
                                ep.feed_id,
                                ep.guid,
                                ep.title,
                                ep.downloaded,
                                AppView::Queue,
                            )),
                            Err(_) => (),
                        }
                    }
                    PodQueueMessage::MoveToPosition(original_index, new_index) => {
                        match new_index < self.queue.len() {
                            true => self.queue.swap(original_index, new_index),
                            false => (),
                        }
                    }
                }
                Command::none()
            }
            Message::PlayerProgressed => {
                match &self.player.sink {
                    Some(sink) => match sink.empty() {
                        true => {
                            let episode_id = match self.queue.get(0) {
                                Some(episode) => Some(episode.id),
                                None => None,
                            };
                            match episode_id {
                                Some(id) => {
                                    self.player = Player::new(Some(id));
                                    self.queue.remove(0);
                                }
                                None => {}
                            }
                        }
                        false => {}
                    },
                    None => {}
                }
                Command::none()
            }
            Message::ThemeChanged(theme) => {
                match create_config(Some(CastironConfig {
                    theme: theme.to_string(),
                })) {
                    Ok(_) => (),
                    Err(_) => (),
                };
                self.theme = theme;
                match &self.castiron_config {
                    Some(config) => {
                        self.castiron_config =
                            Some(Config::new(config.values.clone(), self.theme.clone()))
                    }
                    None => (),
                }
                Command::none()
            }
            Message::SetupCompleted(_) => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(100)).map(|_| Message::PlayerProgressed)
    }

    fn view(&self) -> Element<Message> {
        let main_content = match self.app_view {
            AppView::Feeds => self.feeds.view(),
            AppView::Episodes => self.episodes.view(),
            AppView::EpisodesForShow(id) => {
                let feed_title = match get_feed_by_id(id) {
                    Ok(f) => match f.feed_title {
                        Some(thing) => thing,
                        None => String::new(),
                    },
                    Err(_) => String::new(),
                };
                column![
                    row![
                        button("Back").on_press(Message::ViewFeeds),
                        text(format!("{feed_title}"))
                    ]
                    .padding(10),
                    self.episodes_for_show.view()
                ]
                .spacing(10)
                .align_items(Alignment::Center)
                .into()
            }
            AppView::Queue => match &self.queue.len() {
                0 => container(text("Nothing queued yet."))
                    .center_x()
                    .width(Length::Fill)
                    .into(),
                _ => self.view_queue().into(),
            },
            AppView::Config => match &self.castiron_config {
                Some(config) => config.view().into(),
                None => text("No config to show.").into(),
            },
        };
        column![
            container(row![
                container(
                    column![
                        button(text("Feeds"))
                            .on_press(Message::ViewFeeds)
                            .style(MyButtonStyle::new())
                            .padding(10)
                            .width(Length::Fill),
                        Rule::horizontal(1).style(style_sidebar_rule),
                        button(text("Episodes"))
                            .on_press(Message::ViewEpisodes)
                            .style(MyButtonStyle::new())
                            .padding(10)
                            .width(Length::Fill),
                        Rule::horizontal(1).style(style_sidebar_rule),
                        button(text("Queue"))
                            .on_press(Message::ViewQueue)
                            .style(MyButtonStyle::new())
                            .padding(10)
                            .width(Length::Fill),
                        Rule::horizontal(1).style(style_sidebar_rule),
                        button(text("Config"))
                            .on_press(Message::ViewConfig)
                            .style(MyButtonStyle::new())
                            .padding(10)
                            .width(Length::Fill),
                        column![
                            text_input("add feed", self.feed_to_add.as_str())
                                .on_input(Message::FeedToAddUpdated)
                                .width(Length::Fill),
                            button(text("Add"))
                                .on_press(Message::AddFeed)
                                .padding(10)
                                .width(Length::Fill),
                        ],
                        button(text("Sync"))
                            .on_press(Message::SyncEpisodes)
                            .style(theme::Button::Secondary)
                            .padding(10)
                            .width(Length::Fill),
                        vertical_space(),
                    ]
                    .width(300)
                    .align_items(Alignment::Center),
                )
                .style(style_sidebar),
                main_content
            ])
            .style(style_main_area),
            self.player.view()
        ]
        .into()
    }
}
