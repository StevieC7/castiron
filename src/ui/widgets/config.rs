use crate::{types::config::CastironConfig, ui::gui::Message};
use iced::{
    widget::{container, horizontal_space, pick_list, row, text},
    Alignment, Element, Length, Theme,
};

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
