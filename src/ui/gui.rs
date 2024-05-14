use iced::widget::{button, column, row, text, text_input};
use iced::Theme;
use iced::{executor, Alignment, Application, Command, Element};

use crate::file_handling::config::create_config;
use crate::file_handling::feeds::add_feed_to_database;
use crate::types::config::CastironConfig;
use crate::types::{episodes::Episode as EpisodeData, feeds::FeedMeta, ui::AppView};

use super::widgets::{Config, Episodes, Feed, Feeds};

pub struct AppLayout {
    app_view: AppView,
    feeds: Option<Feeds>,
    castiron_config: Option<Config>,
    feed_to_add: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedsFound(Result<Vec<FeedMeta>, String>),
    EpisodesSynced(Result<Option<Vec<EpisodeData>>, String>),
    ConfigFound(Result<CastironConfig, String>),
    ViewEpisodes,
    ViewFeeds,
    ViewConfig,
    SaveConfig(Option<CastironConfig>),
    AddFeed,
    FeedToAddUpdated(String),
}

impl Application for AppLayout {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                feeds: None,
                app_view: AppView::Feeds,
                castiron_config: None,
                feed_to_add: String::new(),
            },
            Command::batch([
                Command::perform(Feeds::fetch_feeds(), Message::FeedsFound),
                Command::perform(Config::fetch_config(), Message::ConfigFound),
                Command::perform(Episodes::fetch_episodes(), Message::EpisodesSynced),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FeedsFound(feeds) => match feeds {
                Err(_) => Command::none(),
                Ok(data) => {
                    let feed_list = data
                        .iter()
                        .map(|n| Feed::new(n.feed_url.to_owned()))
                        .collect();
                    self.feeds = Some(Feeds::new(feed_list));
                    Command::none()
                }
            },
            Message::EpisodesSynced(episodes) => match episodes {
                Err(_) => Command::none(),
                Ok(_) => Command::none(),
            },
            Message::ConfigFound(config) => match config {
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
                    Ok(_) => Command::perform(Config::fetch_config(), Message::ConfigFound),
                    Err(_) => Command::none(),
                }
            }
            Message::AddFeed => {
                let result = add_feed_to_database(self.feed_to_add.to_owned());
                self.feed_to_add = String::new();
                match result {
                    Ok(_) => Command::perform(Feeds::fetch_feeds(), Message::FeedsFound),
                    Err(_) => Command::none(),
                }
            }
            Message::FeedToAddUpdated(val) => {
                self.feed_to_add = val;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let column = column![
            button(text("Feeds")).on_press(Message::ViewFeeds),
            button(text("Episodes")).on_press(Message::ViewEpisodes),
            button(text("Config")).on_press(Message::ViewConfig),
            text_input("add feed", self.feed_to_add.as_str()).on_input(Message::FeedToAddUpdated),
            button(text("Add")).on_press(Message::AddFeed),
        ]
        .padding(20)
        .align_items(Alignment::Center);
        match self.app_view {
            AppView::Feeds => match self.feeds.as_ref() {
                Some(feeds) => row![column, feeds.view()].into(),
                None => row![column, text("No feeds to show.")].into(),
            },
            AppView::Episodes => row![column, "Episodes go here."].into(),
            AppView::Config => match self.castiron_config.as_ref() {
                Some(config) => row![column, config.view()].into(),
                None => row![column, text("No config to show.")].into(),
            },
        }
    }
}
