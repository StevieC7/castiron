use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox};

use super::widgets::Episode;

pub struct AppLayout {
    value: i32,
    episode: Episode,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for AppLayout {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            episode: Episode::new("yep".to_string()),
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed),
            self.episode.view()
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
