use iced::widget::{button, column, row, text, text_input, vertical_space};
use iced::Theme;
use iced::{executor, Alignment, Application, Command, Element};

use crate::file_handling::config::create_config;
use crate::file_handling::episodes::delete_episode_from_fs;
use crate::file_handling::feeds::add_feed_to_database;
use crate::types::config::CastironConfig;
use crate::types::{episodes::Episode as EpisodeData, feeds::FeedMeta, ui::AppView};

use super::widgets::{Config, Episode, EpisodeList, Feed, FeedList, Player, PlayerMessage};

pub struct AppLayout {
    app_view: AppView,
    feeds: Option<FeedList>,
    episodes: Option<EpisodeList>,
    castiron_config: Option<Config>,
    feed_to_add: String,
    player: Player,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedsLoaded(Result<Vec<FeedMeta>, String>),
    EpisodesLoaded(Result<Option<Vec<EpisodeData>>, String>),
    EpisodesSynced(Result<Option<Vec<EpisodeData>>, String>),
    ConfigLoaded(Result<CastironConfig, String>),
    ViewEpisodes,
    ViewFeeds,
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
    // EpisodesMessage(EpisodesMessage),
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
                feeds: None,
                episodes: None,
                castiron_config: None,
                feed_to_add: String::new(),
                player: Player::new(None),
            },
            Command::batch([
                Command::perform(Config::load_config(), Message::ConfigLoaded),
                Command::perform(FeedList::load_feeds(), Message::FeedsLoaded),
                Command::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
                // TODO: find out how to defer this action until after episode list rendered, and only when config specifies auto download new
                // Command::perform(EpisodeList::sync_episodes(), Message::EpisodesSynced),
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
                            Some(feed_title) => Feed::new(feed_title.to_owned()),
                            None => Feed::new(n.feed_url.to_owned()),
                        })
                        .collect();
                    self.feeds = Some(FeedList::new(feed_list));
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
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                    )
                                })
                                .collect();
                            self.episodes = Some(EpisodeList::new(episode_list));
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
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                    )
                                })
                                .collect();
                            self.episodes = Some(EpisodeList::new(episode_list));
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
            }, // Message::EpisodesMessage(message) => match &mut self.episodes {
               //     Some(episode_list) => {
               //         // episode_list.update(message);
               //         Command::none()
               //     }
               //     None => Command::none(),
               // },
        }
    }

    fn view(&self) -> Element<Message> {
        let column = column![
            button(text("Feeds"))
                .on_press(Message::ViewFeeds)
                .width(200),
            button(text("Episodes"))
                .on_press(Message::ViewEpisodes)
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
            AppView::Feeds => match &self.feeds {
                Some(feeds) => row![column, feeds.view()].into(),
                None => row![column, text("No feeds to show.")].into(),
            },
            AppView::Episodes => match &self.episodes {
                Some(episodes) => row![column, episodes.view()].into(),
                None => row![column, text("No episodes to show.")].into(),
            },
            AppView::Config => match &self.castiron_config {
                Some(config) => row![column, config.view()].into(),
                None => row![column, text("No config to show.")].into(),
            },
        }
    }
}
