use iced::widget::{button, column, row, text};
use iced::Theme;
use iced::{executor, Alignment, Application, Command, Element};

use crate::file_handling::config::create_config;
use crate::types::config::CastironConfig;
use crate::types::{feeds::FeedMeta, ui::AppView};

use super::widgets::{Config, Feed, Feeds};

pub struct AppLayout {
    app_view: AppView,
    feeds: Option<Feeds>,
    castiron_config: Option<Config>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FeedsFound(Result<Vec<FeedMeta>, String>),
    ConfigFound(Result<CastironConfig, String>),
    ViewEpisodes,
    ViewFeeds,
    ViewConfig,
    SaveConfig(Option<CastironConfig>),
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
            },
            Command::batch([
                Command::perform(Feeds::fetch_feeds(), Message::FeedsFound),
                Command::perform(Config::fetch_config(), Message::ConfigFound),
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
        }
    }

    fn view(&self) -> Element<Message> {
        let column = column![
            button(text("Feeds")).on_press(Message::ViewFeeds),
            button(text("Episodes")).on_press(Message::ViewEpisodes),
            button(text("Config")).on_press(Message::ViewConfig),
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
