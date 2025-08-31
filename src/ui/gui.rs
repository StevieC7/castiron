// use url::Url;

use std::io::{BufReader, Read};

use iced::{
    futures::{channel::mpsc, channel::mpsc::Sender, SinkExt, Stream, StreamExt},
    stream,
    widget::{button, column, container, row, text, vertical_space, Rule},
    window, Alignment, Element, Length, Subscription, Task, Theme,
};

use super::widgets::{
    episode_list::EpisodeList,
    feed_list::{FeedList, Message as FeedListMessage},
    player::{Player, PlayerMessage},
};
use crate::{
    file_handling::{
        episodes::{load_episodes, load_episodes_by_id, load_episodes_for_show},
        feeds::load_feeds,
        setup::InitData,
    },
    networking::feeds::sync_episodes,
    types::feeds::FeedMeta,
    ui::widgets::{
        episode::Message as EpisodeMessage, episode_list::Message as EpisodeListMessage, feed,
    },
};

// TODO: convert all view-dependent state to Option type, populate only when view selected
pub struct Castiron {
    app_view: AppView,
    player: Player,
    // TODO: use array of ids instead to fit new pattern better
    queue: Vec<i32>,
    theme: Theme,
}

pub enum AppView {
    Feeds(FeedList),
    EpisodesForShow(EpisodeList),
    Episodes(EpisodeList),
    Queue(EpisodeList),
    Init,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadImages,
    ImageLoaderReady(mpsc::Sender<Message>),
    ImagesLoading,
    FeedList(FeedListMessage),
    EpisodeList(EpisodeListMessage),
    ViewEpisodes,
    ViewFeeds,
    ViewQueue,
    ViewConfig,
    SyncEpisodes,
    PlayerMessage(PlayerMessage),
    InitComplete(InitData),
    InitFailed,
    HandleClose,
}

impl Castiron {
    fn new() -> Self {
        Self {
            // TODO: make an init that does something for real
            app_view: AppView::Init,
            player: Player::new(None),
            queue: Vec::new(),
            theme: Theme::default(),
        }
    }

    // TODO: organize state and messages better
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // TODO: only load the data for the AppView::Init until another view selected
            Message::FeedList(message) => {
                // Intercept the message that sends us to the episode list view for the clicked feed
                if let FeedListMessage::FeedMessage(feed_message) = &message {
                    if let feed::Message::ViewEpisodesForShow(show_id) = feed_message {
                        self.app_view = AppView::EpisodesForShow(EpisodeList::new());
                        Task::perform(load_episodes_for_show(show_id.to_owned()), |res| {
                            Message::EpisodeList(EpisodeListMessage::EpisodesLoaded(res, false))
                        })
                    } else {
                        match &mut self.app_view {
                            AppView::Feeds(feeds) => {
                                let task = feeds.update(message).map(|msg| Message::FeedList(msg));
                                task
                            }
                            _ => Task::none(),
                        }
                    }
                } else {
                    match &mut self.app_view {
                        AppView::Feeds(feeds) => {
                            let task = feeds.update(message).map(|msg| Message::FeedList(msg));
                            task
                        }
                        _ => Task::none(),
                    }
                }
            }
            Message::EpisodeList(message) => match &message {
                EpisodeListMessage::EpisodeMessage((id, msg)) => match msg {
                    // TODO: queue component should handle this
                    EpisodeMessage::AddToQueue => {
                        self.queue.push(id.clone());
                        Task::none()
                    }
                    EpisodeMessage::Play => {
                        self.player = Player::new(Some(id.clone()));
                        let found_idx = self.queue.iter().position(|episode_id| episode_id == id);
                        match found_idx {
                            Some(idx) => {
                                self.queue.remove(idx);
                            }
                            None => {}
                        }
                        Task::none()
                    }
                    _ => match &mut self.app_view {
                        AppView::Episodes(episodes) => {
                            let task = episodes
                                .update(message)
                                .map(|msg| Message::EpisodeList(msg));
                            task
                        }
                        AppView::EpisodesForShow(episodes) => {
                            let task = episodes
                                .update(message)
                                .map(|msg| Message::EpisodeList(msg));
                            task
                        }
                        _ => Task::none(),
                    },
                },
                _ => match &mut self.app_view {
                    AppView::Episodes(episodes) => {
                        let task = episodes
                            .update(message)
                            .map(|msg| Message::EpisodeList(msg));
                        task
                    }
                    AppView::EpisodesForShow(episodes) => {
                        let task = episodes
                            .update(message)
                            .map(|msg| Message::EpisodeList(msg));
                        task
                    }
                    AppView::Queue(episodes) => {
                        let task = episodes
                            .update(message)
                            .map(|msg| Message::EpisodeList(msg));
                        task
                    }
                    _ => Task::none(),
                },
            },
            Message::ViewFeeds => {
                self.app_view = AppView::Feeds(FeedList::new());
                Task::perform(load_feeds(), |res| {
                    Message::FeedList(FeedListMessage::FeedsLoaded(res))
                })
            }
            Message::ViewEpisodes => {
                self.app_view = AppView::Episodes(EpisodeList::new());
                Task::perform(load_episodes(), |res| {
                    Message::EpisodeList(EpisodeListMessage::EpisodesLoaded(res, false))
                })
            }
            Message::ViewQueue => {
                self.app_view = AppView::Queue(EpisodeList::new());
                // TODO: call function for getting episodes by array of ids
                Task::perform(load_episodes_by_id(self.queue.clone()), |res| {
                    Message::EpisodeList(EpisodeListMessage::EpisodesLoaded(res, true))
                })
            }
            Message::ImageLoaderReady(sender) => {
                Task::perform(send_load_image_message(sender), |_| Message::ImagesLoading)
            }
            Message::InitComplete(data) => {
                self.app_view = AppView::Feeds(FeedList::new());
                Task::done(Message::FeedList(FeedListMessage::FeedsLoaded(Ok(
                    data.feeds
                ))))
            }
            Message::PlayerMessage(message) => {
                match message {
                    PlayerMessage::Progress => match &self.player.sink {
                        Some(sink) => match sink.empty() {
                            true => {
                                let episode_id = match self.queue.get(0) {
                                    Some(id) => Some(id),
                                    None => None,
                                };
                                match episode_id {
                                    Some(id) => {
                                        self.player = Player::new(Some(*id));
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
            Message::SyncEpisodes => Task::perform(sync_episodes(), |_| Message::ViewFeeds),
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            self.player.subscription(),
            match self.app_view {
                AppView::Feeds(_) => Subscription::run(image_loader),
                _ => Subscription::none(),
            },
            window::close_requests().map(|_| Message::HandleClose),
        ])
    }

    pub fn view(&self) -> Element<Message> {
        let main_content = match &self.app_view {
            AppView::Init => container(text("Loading..."))
                .padding(20)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
            AppView::Feeds(feeds) => feeds.view().map(Message::FeedList),
            AppView::Episodes(episodes) => episodes.view().map(Message::EpisodeList),
            AppView::EpisodesForShow(episodes) => episodes.view().map(Message::EpisodeList),
            AppView::Queue(episodes) => episodes.view().map(Message::EpisodeList),
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
                            // column![
                            //     text_input("add feed", self.feed_to_add.as_str())
                            //         .on_input(Message::FeedToAddUpdated)
                            //         .width(Length::Fill),
                            //     button(text("Add"))
                            //         .on_press(Message::AddFeed)
                            //         .padding(10)
                            //         .width(Length::Fill),
                            // ],
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

fn image_loader() -> impl Stream<Item = Message> {
    stream::channel(256, |mut output| async move {
        // Create channel
        let (sender, mut receiver) = mpsc::channel(256);

        // Send the sender back to the application
        let _ = output.send(Message::ImageLoaderReady(sender)).await;

        loop {
            // Read next input sent from `Application`
            let input = receiver.select_next_some().await;

            match input {
                // TODO: update enum variant to accept list of images to load
                Message::LoadImages => {
                    // Do some async work...
                    let feeds_meta = load_feeds().await.unwrap_or(Vec::new());
                    for meta in feeds_meta {
                        let FeedMeta {
                            id,
                            image_file_path,
                            ..
                        } = meta;
                        if let Some(file_path) = image_file_path {
                            let file = std::fs::File::open(file_path);
                            match file {
                                Ok(f) => {
                                    let byte_collection: Result<Vec<u8>, std::io::Error> =
                                        BufReader::new(f).bytes().collect();
                                    match byte_collection {
                                        Err(_) => todo!(),
                                        Ok(bytes) => {
                                            let _ = output
                                                .send(Message::FeedList(
                                                    FeedListMessage::ImageLoaded(id, bytes),
                                                ))
                                                .await;
                                        }
                                    }
                                }
                                Err(_) => {
                                    let _ = output
                                        .send(Message::FeedList(FeedListMessage::ImageFail))
                                        .await;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    })
}

async fn send_load_image_message(mut sender: Sender<Message>) {
    let _ = sender.send(Message::LoadImages).await;
}
