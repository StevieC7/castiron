use super::gui::Message;
use iced::widget::text;
use iced::Element;

pub struct Episode {
    title: String,
}

impl Episode {
    pub fn new(title: String) -> Self {
        Self { title }
    }
    pub fn view(&self) -> Element<Message> {
        text(&self.title).into()
    }
}
