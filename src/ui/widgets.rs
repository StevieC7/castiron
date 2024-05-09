use super::gui::Message;
use iced::widget::{row, text};
use iced::Element;

#[derive(Clone)]
pub struct Episode {
    feed_title: String,
    episode_title: String,
    date: String,
    played_seconds: i32,
    played: bool,
}

impl Episode {
    pub fn new(
        feed_title: String,
        episode_title: String,
        date: String,
        played_seconds: i32,
        played: bool,
    ) -> Self {
        let episode = Self {
            feed_title,
            episode_title,
            date,
            played_seconds,
            played,
        };
        episode
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenPlayer => {
                println!("This would activate opening the player.")
            }
            _ => (),
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![text("Yes")].padding(20).spacing(20).into()
    }
}
