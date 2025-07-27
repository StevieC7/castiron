// use url::Url;

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
        config::{convert_theme_string_to_enum, create_config},
        episodes::{delete_episode_from_fs, get_episode_by_id, get_episodes_by_feed_id},
        feeds::{
            add_feed_to_database, delete_associated_episodes_and_xml, get_feed_by_id, load_feeds,
        },
        queue::save_queue,
        setup::InitData,
    },
    types::{config::CastironConfig, episodes::Episode as EpisodeData, feeds::FeedMeta},
    ui::widgets::{feed, feed_list},
};

// TODO: convert all view-dependent state to Option type, populate only when view selected
pub struct Castiron {
    app_view: AppView,
    episodes: EpisodeList,
    episodes_for_show: EpisodeList,
    castiron_config: Option<Config>,
    feed_to_add: String,
    player: Player,
    queue: Vec<Episode>,
    theme: Theme,
}

pub enum AppView {
    Feeds(FeedList),
    Episodes,
    EpisodesForShow(i32),
    Config,
    Queue,
    Init,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedList(feed_list::Message),
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
            episodes: EpisodeList::new(Vec::new()),
            episodes_for_show: EpisodeList::new(Vec::new()),
            castiron_config: None,
            feed_to_add: String::new(),
            player: Player::new(None),
            queue: Vec::new(),
            theme: Theme::default(),
        }
    }

    // TODO: turn queue into component
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
    // TODO: turn queue into component
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

    // TODO: organize state and messages better
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // TODO: only load the data for the AppView::Init until another view selected
            Message::FeedList(message) => {
                if let AppView::Feeds(feeds) = &mut self.app_view {
                    let task = feeds.update(message).map(|msg| Message::FeedList(msg));
                    task
                } else {
                    Task::none()
                }
            }
            Message::ViewFeeds => {
                self.app_view = AppView::Feeds(FeedList::new());
                Task::perform(load_feeds(), |res| {
                    Message::FeedList(feed_list::Message::FeedsLoaded(res))
                })
            }
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.player.subscription(),
            window::close_requests().map(|_| Message::HandleClose),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        match &self.app_view {
            AppView::Init => button("Feeds").on_press(Message::ViewFeeds).into(),
            AppView::Feeds(feeds) => feeds.view().map(Message::FeedList),
            AppView::Episodes => todo!(),
            AppView::EpisodesForShow(_) => todo!(),
            AppView::Config => todo!(),
            AppView::Queue => todo!(),
        }
        // let main_content = match self.app_view {
        //     AppView::Feeds => self.feeds.view(),
        //     AppView::Episodes => self.episodes.view(),
        //     AppView::EpisodesForShow(id) => {
        //         let feed = get_feed_by_id(id);
        //         match feed {
        //             Ok(f) => column![
        //                 row![
        //                     button("Back").on_press(Message::ViewFeeds),
        //                     text(format!("{}", f.feed_title.unwrap_or(String::new())))
        //                 ]
        //                 .padding(10),
        //                 self.episodes_for_show.view()
        //             ]
        //             .spacing(10)
        //             .into(),
        //             Err(_) => text("Error loading").into(),
        //         }
        //     }
        //     AppView::Queue => match &self.queue.len() {
        //         0 => container(text("Queue is empty."))
        //             .padding(20)
        //             .center_x(Length::Fill)
        //             .into(),
        //         _ => self.view_queue().into(),
        //     },
        //     AppView::Config => match &self.castiron_config {
        //         Some(config) => config.view().into(),
        //         None => container(text("Config does not exist."))
        //             .padding(20)
        //             .center_x(Length::Fill)
        //             .into(),
        //     },
        //     AppView::Init => container(text("Loading..."))
        //         .padding(20)
        //         .center_x(Length::Fill)
        //         .into(),
        // };
        // match self.app_view {
        //     AppView::Init => container(text("Loading..."))
        //         .padding(20)
        //         .center_x(Length::Fill)
        //         .center_y(Length::Fill)
        //         .into(),
        //     _ => column![
        //         container(row![
        //             container(
        //                 column![
        //                     button(text("Feeds"))
        //                         .on_press(Message::ViewFeeds)
        //                         .padding(10)
        //                         .width(Length::Fill),
        //                     Rule::horizontal(1),
        //                     button(text("Episodes"))
        //                         .on_press(Message::ViewEpisodes)
        //                         .padding(10)
        //                         .width(Length::Fill),
        //                     Rule::horizontal(1),
        //                     button(text("Queue"))
        //                         .on_press(Message::ViewQueue)
        //                         .padding(10)
        //                         .width(Length::Fill),
        //                     Rule::horizontal(1),
        //                     button(text("Config"))
        //                         .on_press(Message::ViewConfig)
        //                         .padding(10)
        //                         .width(Length::Fill),
        //                     column![
        //                         text_input("add feed", self.feed_to_add.as_str())
        //                             .on_input(Message::FeedToAddUpdated)
        //                             .width(Length::Fill),
        //                         button(text("Add"))
        //                             .on_press(Message::AddFeed)
        //                             .padding(10)
        //                             .width(Length::Fill),
        //                     ],
        //                     button(text("Sync"))
        //                         .on_press(Message::SyncEpisodes)
        //                         .padding(10)
        //                         .width(Length::Fill),
        //                     vertical_space(),
        //                 ]
        //                 .width(300)
        //                 .align_x(Alignment::Center),
        //             ),
        //             main_content
        //         ]),
        //         self.player.view()
        //     ]
        //     .into(),
        // }
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
