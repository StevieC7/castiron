use std::time::Duration;

use iced::widget::{button, column, row, text, text_input, vertical_space, Column};
use iced::{executor, Alignment, Application, Command, Element};
use iced::{time, Subscription, Theme};

use crate::file_handling::config::create_config;
use crate::file_handling::episodes::{delete_episode_from_fs, get_episode_by_guid};
use crate::file_handling::feeds::{add_feed_to_database, delete_feed_from_database_only};
use crate::types::config::CastironConfig;
use crate::types::{episodes::Episode as EpisodeData, feeds::FeedMeta};

use super::widgets::{Config, Episode, EpisodeList, Feed, FeedList, Player, PlayerMessage};

pub struct AppLayout {
    app_view: AppView,
    feeds: FeedList,
    episodes: EpisodeList,
    castiron_config: Option<Config>,
    feed_to_add: String,
    player: Player,
    queue: Vec<Episode>,
}

pub enum AppView {
    Feeds,
    Episodes,
    Config,
    Queue,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedsLoaded(Result<Vec<FeedMeta>, String>),
    EpisodesLoaded(Result<Option<Vec<EpisodeData>>, String>),
    EpisodesSynced(Result<Option<Vec<EpisodeData>>, String>),
    ConfigLoaded(Result<CastironConfig, String>),
    ViewEpisodes,
    ViewFeeds,
    ViewQueue,
    ViewConfig,
    SaveConfig(Option<CastironConfig>),
    AddFeed,
    FeedToAddUpdated(String),
    SyncEpisodes,
    DownloadEpisode(String),
    DeleteEpisode(String),
    EpisodeDownloaded(Result<(), String>),
    PlayEpisode(String),
    PlayerMessage(PlayerMessage),
    PlayerProgressed,
    PodQueueMessage(PodQueueMessage),
    UnfollowFeed(i32),
}

#[derive(Debug, Clone)]
pub enum PodQueueMessage {
    RemoveFromQueue(i32),
    AddToQueue(String),
}

impl AppLayout {
    pub fn view_queue(&self) -> Element<Message> {
        self.queue
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(row!(
                    content.view(),
                    button(text("Rm from queue")).on_press(Message::PodQueueMessage(
                        PodQueueMessage::RemoveFromQueue(content.id)
                    ))
                ))
            })
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
                castiron_config: None,
                feed_to_add: String::new(),
                player: Player::new(None),
                queue: Vec::new(),
            },
            Command::batch([
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
        Theme::CatppuccinMocha
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
                                    println!("Episode {} has id {}", n.title, n.id);
                                    Episode::new(
                                        n.id,
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                    )
                                })
                                .collect();
                            self.episodes = EpisodeList::new(episode_list);
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
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
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
                    self.castiron_config = Some(Config::new(data));
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
            Message::ViewQueue => {
                self.app_view = AppView::Queue;
                Command::none()
            }
            Message::ViewConfig => {
                self.app_view = AppView::Config;
                Command::none()
            }
            Message::SaveConfig(config) => {
                let update_config_result = create_config(config);
                match update_config_result {
                    Ok(_) => Command::perform(Config::load_config(), Message::ConfigLoaded),
                    Err(_) => Command::none(),
                }
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
                Ok(_) => Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
                Err(e) => {
                    println!("Error downloading episode: {e}");
                    Command::none()
                }
            },
            Message::PlayEpisode(guid) => {
                self.player = Player::new(Some(guid));
                Command::none()
            }
            Message::PlayerMessage(message) => {
                self.player.update(message);
                Command::none()
            }
            Message::DeleteEpisode(guid) => match delete_episode_from_fs(guid) {
                Ok(_) => Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
                Err(e) => {
                    eprintln!("Error deleting episode: {:?}", e);
                    Command::none()
                }
            },
            Message::UnfollowFeed(id) => match delete_feed_from_database_only(id) {
                Ok(_) => Command::perform(FeedList::load_feeds(), Message::FeedsLoaded),
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
                    PodQueueMessage::AddToQueue(guid) => {
                        let episode = get_episode_by_guid(&guid);
                        match episode {
                            Ok(ep) => self.queue.push(Episode::new(
                                ep.id,
                                ep.guid,
                                ep.title,
                                ep.downloaded,
                            )),
                            Err(_) => (),
                        }
                    }
                }
                Command::none()
            }
            Message::PlayerProgressed => {
                match &self.player.sink {
                    Some(sink) => match sink.empty() {
                        true => {
                            self.queue.remove(0);
                            let guid = match self.queue.get(0) {
                                Some(episode) => episode.guid.to_owned(),
                                None => String::new(),
                            };
                            self.player = Player::new(Some(guid));
                        }
                        false => {}
                    },
                    None => {}
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(100)).map(|_| Message::PlayerProgressed)
    }

    fn view(&self) -> Element<Message> {
        let column = column![
            button(text("Feeds"))
                .on_press(Message::ViewFeeds)
                .width(200),
            button(text("Episodes"))
                .on_press(Message::ViewEpisodes)
                .width(200),
            button(text("Queue"))
                .on_press(Message::ViewQueue)
                .width(200),
            button(text("Config"))
                .on_press(Message::ViewConfig)
                .width(200),
            text_input("add feed", self.feed_to_add.as_str())
                .on_input(Message::FeedToAddUpdated)
                .width(200),
            button(text("Add")).on_press(Message::AddFeed).width(200),
            button(text("Sync"))
                .on_press(Message::SyncEpisodes)
                .width(200),
            vertical_space(),
            self.player.view()
        ]
        .padding(20)
        .width(300)
        .align_items(Alignment::Center);
        match self.app_view {
            AppView::Feeds => row![column, self.feeds.view()].into(),
            AppView::Episodes => row![column, self.episodes.view()].into(),
            AppView::Config => match &self.castiron_config {
                Some(config) => row![column, config.view()].into(),
                None => row![column, text("No config to show.")].into(),
            },
            AppView::Queue => match &self.queue.len() {
                0 => row![column, text("Nothing queued yet.")].into(),
                _ => row![column, self.view_queue()].into(),
            },
        }
    }
}
