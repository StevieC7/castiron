use crate::file_handling::feeds::get_feed_list_database;
use crate::types::config::CastironConfig;
use crate::types::feeds::FeedMeta;

use super::gui::Message;
use iced::widget::container::Appearance;
use iced::widget::{container, row, text, Column, Toggler};
use iced::{Border, Color, Element, Shadow};

#[derive(Clone)]
pub struct Feeds {
    feeds: Vec<Feed>,
}

impl Feeds {
    pub fn new(feeds: Vec<Feed>) -> Self {
        Self { feeds }
    }
    pub fn view(&self) -> Element<Message> {
        let col = Column::new();
        let feeds: Element<Message> = self
            .feeds
            .iter()
            .fold(Column::new().spacing(10), |col, content| {
                col.push(content.view())
            })
            .into();
        col.push(feeds).into()
    }
    pub async fn fetch_feeds() -> Result<Vec<FeedMeta>, String> {
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
pub struct Feed {
    feed_url: String,
}

impl Feed {
    pub fn new(feed_url: String) -> Self {
        Self { feed_url }
    }
    pub fn view(&self) -> Element<Message> {
        container(row!(text(self.feed_url.to_owned())))
            .style(Appearance {
                background: Some(iced::Background::Color(Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                })),
                text_color: None,
                border: Border {
                    color: Color::default(),
                    width: 0.0,
                    radius: [5.0, 5.0, 5.0, 5.0].into(),
                },
                shadow: Shadow::default(),
            })
            .max_width(500)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

#[derive(Clone)]
pub struct Config {
    values: Option<CastironConfig>,
}

impl Config {
    pub fn new(values: Option<CastironConfig>) -> Self {
        Self { values }
    }

    pub fn view(&self) -> Element<Message> {
        match self.values.to_owned() {
            Some(vals) => {
                let column = Column::new();
                column
                    .push(Toggler::new(
                        String::from("Automatically download new episodes?"),
                        vals.auto_dl_new,
                        move |n| {
                            Message::SaveConfig(CastironConfig {
                                auto_dl_new: n,
                                auto_rm_after_listen: vals.auto_rm_after_listen,
                                dark_mode: vals.dark_mode,
                            })
                        },
                    ))
                    .into()
            }
            None => container(text("Config setup failed.")).into(),
        }
    }
}
