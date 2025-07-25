use url::Url;

use iced::{
    advanced::image::Handle,
    widget::{
        button, column, container, row, text, text_input, vertical_space, Column, Rule, Scrollable,
    },
    window, Alignment, Element, Length, Subscription, Task, Theme,
};

use super::widgets::{
    config::Config,
    episode::Episode,
    episode_list::EpisodeList,
    feed::Feed,
    feed_list::FeedList,
    player::{Player, PlayerMessage},
};
use crate::{
    file_handling::{
        config::{convert_theme_string_to_enum, create_config, load_or_create_config},
        episodes::{
            delete_episode_from_fs, get_episode_by_id, get_episode_list_database,
            get_episodes_by_feed_id,
        },
        feeds::{
            add_feed_to_database, delete_associated_episodes_and_xml, get_feed_by_id,
            get_feed_list_database,
        },
        queue::{get_queue_database, save_queue},
        setup::InitData,
    },
    types::{config::CastironConfig, episodes::Episode as EpisodeData, feeds::FeedMeta},
};

pub struct Castiron {
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
    Init,
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
    FeedsLoaded(Result<Vec<FeedMeta>, String>),
    EpisodesLoaded(Result<Option<Vec<EpisodeData>>, String>),
    EpisodesSynced(Result<Option<Vec<EpisodeData>>, String>),
    EpisodeDownloaded(Result<(), String>),
    FeedToAddUpdated(String),
    PlayerMessage(PlayerMessage),
    PodQueueMessage(PodQueueMessage),
    ThemeChanged(Theme),
    InitComplete(InitData),
    InitFailed,
    HandleClose,
}

#[derive(Debug, Clone)]
pub enum PodQueueMessage {
    RemoveFromQueue(i32),
    AddToQueue(i32),
    MoveToPosition(usize, usize),
}

impl Castiron {
    fn new() -> Self {
        Self {
            app_view: AppView::Init,
            feeds: FeedList::new(Vec::new()),
            episodes: EpisodeList::new(Vec::new()),
            episodes_for_show: EpisodeList::new(Vec::new()),
            castiron_config: None,
            feed_to_add: String::new(),
            player: Player::new(None),
            queue: Vec::new(),
            theme: Theme::default(),
        }
    }

    pub fn update_queue(&mut self) {
        let new_queue: Vec<Episode> = self
            .queue
            .iter()
            .map(|episode| {
                let updated_episode = get_episode_by_id(episode.id);
                match updated_episode {
                    Ok(u_episode) => {
                        let handle = match get_feed_by_id(u_episode.feed_id) {
                            Ok(feed) => match feed.image_file_path {
                                Some(path) => Some(Handle::from_path(path)),
                                None => None,
                            },
                            Err(_) => None,
                        };
                        Episode::new(
                            u_episode.id,
                            u_episode.feed_id,
                            u_episode.guid,
                            u_episode.title,
                            u_episode.downloaded,
                            AppView::Queue,
                            handle,
                        )
                    }
                    Err(_) => {
                        let handle = match get_feed_by_id(episode.feed_id) {
                            Ok(feed) => match feed.image_file_path {
                                Some(path) => Some(Handle::from_path(path)),
                                None => None,
                            },
                            Err(_) => None,
                        };
                        Episode::new(
                            episode.id,
                            episode.feed_id,
                            episode.guid.to_owned(),
                            episode.title.to_owned(),
                            episode.downloaded,
                            AppView::Queue,
                            handle,
                        )
                    }
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
                        column![
                            button(text("Move Up"))
                                .on_press(Message::PodQueueMessage(
                                    PodQueueMessage::MoveToPosition(
                                        col_len.wrapping_sub(1),
                                        col_len.wrapping_sub(2)
                                    )
                                ))
                                .width(100)
                                .height(Length::Fill),
                            button(text("X"))
                                .on_press(Message::PodQueueMessage(
                                    PodQueueMessage::RemoveFromQueue(content.id)
                                ))
                                .width(100)
                                .height(Length::Fill),
                            button(text("Move Down"))
                                .on_press(Message::PodQueueMessage(
                                    PodQueueMessage::MoveToPosition(
                                        col_len.wrapping_sub(1),
                                        col_len
                                    )
                                ))
                                .width(100)
                                .height(Length::Fill)
                        ]
                    ]
                    .height(100),
                )
            });
        Scrollable::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // TOOD: implement state and UI for loading until init complete
            Message::InitComplete(init_data) => {
                self.feeds = FeedList::new(
                    init_data
                        .feeds
                        .iter()
                        .map(|n| match &n.image_file_path {
                            Some(file_path) => match &n.feed_title {
                                Some(feed_title) => Feed::new(
                                    n.id,
                                    feed_title.to_owned(),
                                    Some(Handle::from_path(file_path.to_owned())),
                                ),
                                None => Feed::new(
                                    n.id,
                                    n.feed_url.to_owned(),
                                    Some(Handle::from_path(file_path.to_owned())),
                                ),
                            },
                            None => Feed::new(n.id, Default::default(), Default::default()),
                        })
                        .collect(),
                );
                self.episodes = EpisodeList::new(
                    init_data
                        .episodes
                        .iter()
                        .map(|n| {
                            let handle = match get_feed_by_id(n.feed_id) {
                                Ok(feed) => match feed.image_file_path {
                                    Some(path) => Some(Handle::from_path(path)),
                                    None => None,
                                },
                                Err(_) => None,
                            };
                            Episode::new(
                                n.id,
                                n.feed_id,
                                n.guid.to_owned(),
                                n.title.to_owned(),
                                n.downloaded,
                                AppView::Episodes,
                                handle,
                            )
                        })
                        .collect(),
                );
                self.castiron_config = Some(Config {
                    values: init_data.config.to_owned(),
                    theme: convert_theme_string_to_enum(init_data.config.to_owned().theme),
                });
                self.queue = init_data
                    .queue
                    .iter()
                    .map(|e| {
                        let handle = match get_feed_by_id(e.feed_id) {
                            Ok(feed) => match feed.image_file_path {
                                Some(path) => Some(Handle::from_path(path)),
                                None => None,
                            },
                            Err(_) => None,
                        };
                        Episode::new(
                            e.id,
                            e.feed_id,
                            e.guid.to_owned(),
                            e.title.to_owned(),
                            e.downloaded,
                            AppView::Queue,
                            handle,
                        )
                    })
                    .collect();
                self.theme = convert_theme_string_to_enum(init_data.config.theme);
                self.app_view = AppView::Feeds;
                Task::none()
            }
            Message::InitFailed => Task::none(),
            Message::HandleClose => {
                let ids = self.queue.iter().map(|n| n.id).collect();
                save_queue(ids).unwrap();
                window::get_latest().and_then(window::close)
            }
            Message::FeedsLoaded(feeds) => match feeds {
                Err(_) => Task::none(),
                Ok(data) => {
                    let feed_list = data
                        .iter()
                        .map(|n| match &n.image_file_path {
                            Some(file_path) => match &n.feed_title {
                                Some(feed_title) => Feed::new(
                                    n.id,
                                    feed_title.to_owned(),
                                    Some(Handle::from_path(file_path.to_owned())),
                                ),
                                None => Feed::new(
                                    n.id,
                                    n.feed_url.to_owned(),
                                    Some(Handle::from_path(file_path.to_owned())),
                                ),
                            },
                            None => Feed::new(n.id, Default::default(), Default::default()),
                        })
                        .collect();
                    self.feeds = FeedList::new(feed_list);
                    Task::none()
                }
            },
            Message::EpisodesLoaded(episodes) => match episodes {
                Err(e) => {
                    eprintln!("Episode loading failed: {:?}", e);
                    Task::none()
                }
                Ok(data) => {
                    match data {
                        Some(found) => {
                            let episode_list = found
                                .iter()
                                .map(|n| {
                                    let handle = match get_feed_by_id(n.feed_id) {
                                        Ok(feed) => match feed.image_file_path {
                                            Some(path) => Some(Handle::from_path(path)),
                                            None => None,
                                        },
                                        Err(_) => None,
                                    };
                                    Episode::new(
                                        n.id,
                                        n.feed_id,
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                        AppView::Episodes,
                                        handle,
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
                                                let handle = match get_feed_by_id(n.feed_id) {
                                                    Ok(feed) => match feed.image_file_path {
                                                        Some(path) => Some(Handle::from_path(path)),
                                                        None => None,
                                                    },
                                                    Err(_) => None,
                                                };
                                                Episode::new(
                                                    n.id,
                                                    n.feed_id,
                                                    n.guid.to_owned(),
                                                    n.title.to_owned(),
                                                    n.downloaded,
                                                    AppView::EpisodesForShow(id),
                                                    handle,
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
                    Task::none()
                }
            },
            Message::EpisodesSynced(episodes) => match episodes {
                Err(e) => {
                    eprintln!("Episode sync failed: {:?}", e);
                    Task::none()
                }
                Ok(data) => {
                    match data {
                        Some(found) => {
                            let episode_list = found
                                .iter()
                                .map(|n| {
                                    let handle = match get_feed_by_id(n.feed_id) {
                                        Ok(feed) => match feed.image_file_path {
                                            Some(path) => Some(Handle::from_path(path)),
                                            None => None,
                                        },
                                        Err(_) => None,
                                    };
                                    Episode::new(
                                        n.id,
                                        n.feed_id,
                                        n.guid.to_owned(),
                                        n.title.to_owned(),
                                        n.downloaded,
                                        AppView::Episodes,
                                        handle,
                                    )
                                })
                                .collect();
                            self.episodes = EpisodeList::new(episode_list);
                        }
                        None => {}
                    };
                    Task::batch([
                        Task::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
                        Task::perform(FeedList::load_feeds(), Message::FeedsLoaded),
                    ])
                }
            },
            Message::ViewEpisodes => {
                self.app_view = AppView::Episodes;
                Task::none()
            }
            Message::ViewFeeds => {
                self.app_view = AppView::Feeds;
                Task::none()
            }
            Message::ViewEpisodesForShow(id) => {
                let episodes_for_show_result = get_episodes_by_feed_id(id);
                match episodes_for_show_result {
                    Ok(episodes_for_show) => {
                        let episode_list = episodes_for_show
                            .iter()
                            .map(|n| {
                                let handle = match get_feed_by_id(n.feed_id) {
                                    Ok(feed) => match feed.image_file_path {
                                        Some(path) => Some(Handle::from_path(path)),
                                        None => None,
                                    },
                                    Err(_) => None,
                                };
                                Episode::new(
                                    n.id,
                                    n.feed_id,
                                    n.guid.to_owned(),
                                    n.title.to_owned(),
                                    n.downloaded,
                                    AppView::EpisodesForShow(id),
                                    handle,
                                )
                            })
                            .collect();
                        self.episodes_for_show = EpisodeList::new(episode_list);
                    }
                    Err(_) => {}
                }
                self.app_view = AppView::EpisodesForShow(id);
                Task::none()
            }
            Message::ViewQueue => {
                self.app_view = AppView::Queue;
                Task::none()
            }
            Message::ViewConfig => {
                self.app_view = AppView::Config;
                Task::none()
            }
            Message::AddFeed => {
                if self.feed_to_add == String::new() {
                    Task::none()
                } else if let Err(_) = Url::parse(self.feed_to_add.as_str()) {
                    // TODO: warn user that URL is invalid
                    Task::none()
                } else {
                    let result = add_feed_to_database(self.feed_to_add.to_owned());
                    self.feed_to_add = String::new();
                    match result {
                        Ok(_) => {
                            Task::perform(EpisodeList::sync_episodes(), Message::EpisodesSynced)
                        }
                        Err(_) => Task::none(),
                    }
                }
            }
            Message::FeedToAddUpdated(val) => {
                self.feed_to_add = val;
                Task::none()
            }
            Message::SyncEpisodes => {
                Task::perform(EpisodeList::sync_episodes(), Message::EpisodesSynced)
            }
            Message::DownloadEpisode(guid) => Task::perform(
                Episode::download_single_episode(guid),
                Message::EpisodeDownloaded,
            ),
            Message::EpisodeDownloaded(result) => match result {
                Ok(_) => {
                    self.update_queue();
                    Task::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded)
                }
                Err(e) => {
                    eprintln!("Error downloading episode: {e}");
                    Task::none()
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
                Task::none()
            }
            Message::PlayerMessage(message) => {
                match message {
                    PlayerMessage::Progress => match &self.player.sink {
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
                                };
                            }
                            false => {}
                        },
                        None => {}
                    },
                    _ => {}
                };
                self.player.update(message);
                Task::none()
            }
            Message::DeleteEpisode(guid) => match delete_episode_from_fs(guid) {
                Ok(_) => {
                    self.update_queue();
                    Task::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded)
                }
                Err(e) => {
                    eprintln!("Error deleting episode: {:?}", e);
                    Task::none()
                }
            },
            Message::UnfollowFeed(id) => match delete_associated_episodes_and_xml(id) {
                Ok(_) => Task::batch([
                    Task::perform(FeedList::load_feeds(), Message::FeedsLoaded),
                    Task::perform(EpisodeList::load_episodes(), Message::EpisodesLoaded),
                ]),
                Err(e) => {
                    eprintln!("Error deleting feed: {:?}", e);
                    Task::none()
                }
            },
            Message::PodQueueMessage(pod_queue_message) => {
                match pod_queue_message {
                    PodQueueMessage::RemoveFromQueue(id) => {
                        let position = self.queue.iter().position(|pod| pod.id == id);
                        match position {
                            Some(index) => {
                                self.queue.remove(index);
                            }
                            None => {}
                        }
                    }
                    PodQueueMessage::AddToQueue(id) => {
                        let episode = get_episode_by_id(id);
                        match episode {
                            Ok(ep) => {
                                let handle = match get_feed_by_id(ep.feed_id) {
                                    Ok(feed) => match feed.image_file_path {
                                        Some(path) => Some(Handle::from_path(path)),
                                        None => None,
                                    },
                                    Err(_) => None,
                                };
                                self.queue.push(Episode::new(
                                    ep.id,
                                    ep.feed_id,
                                    ep.guid,
                                    ep.title,
                                    ep.downloaded,
                                    AppView::Queue,
                                    handle,
                                ))
                            }
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
                Task::none()
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
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.player.subscription(),
            window::close_requests().map(|_| Message::HandleClose),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        let main_content = match self.app_view {
            AppView::Feeds => self.feeds.view(),
            AppView::Episodes => self.episodes.view(),
            AppView::EpisodesForShow(id) => {
                let feed = get_feed_by_id(id);
                match feed {
                    Ok(f) => column![
                        row![
                            button("Back").on_press(Message::ViewFeeds),
                            text(format!("{}", f.feed_title.unwrap_or(String::new())))
                        ]
                        .padding(10),
                        self.episodes_for_show.view()
                    ]
                    .spacing(10)
                    .into(),
                    Err(_) => text("Error loading").into(),
                }
            }
            AppView::Queue => match &self.queue.len() {
                0 => container(text("Queue is empty."))
                    .padding(20)
                    .center_x(Length::Fill)
                    .into(),
                _ => self.view_queue().into(),
            },
            AppView::Config => match &self.castiron_config {
                Some(config) => config.view().into(),
                None => container(text("Config does not exist."))
                    .padding(20)
                    .center_x(Length::Fill)
                    .into(),
            },
            AppView::Init => container(text("Loading..."))
                .padding(20)
                .center_x(Length::Fill)
                .into(),
        };
        match self.app_view {
            AppView::Init => container(text("Loading..."))
                .padding(20)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            _ => column![
                container(row![
                    container(
                        column![
                            button(text("Feeds"))
                                .on_press(Message::ViewFeeds)
                                .padding(10)
                                .width(Length::Fill),
                            Rule::horizontal(1),
                            button(text("Episodes"))
                                .on_press(Message::ViewEpisodes)
                                .padding(10)
                                .width(Length::Fill),
                            Rule::horizontal(1),
                            button(text("Queue"))
                                .on_press(Message::ViewQueue)
                                .padding(10)
                                .width(Length::Fill),
                            Rule::horizontal(1),
                            button(text("Config"))
                                .on_press(Message::ViewConfig)
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
                                .padding(10)
                                .width(Length::Fill),
                            vertical_space(),
                        ]
                        .width(300)
                        .align_x(Alignment::Center),
                    ),
                    main_content
                ]),
                self.player.view()
            ]
            .into(),
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

impl Default for Castiron {
    fn default() -> Self {
        Self::new()
    }
}
