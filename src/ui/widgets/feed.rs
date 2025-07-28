use iced::{
    advanced::image::Handle,
    widget::{button, container, image, row, text},
    Element, Length,
};

#[derive(Debug, Clone)]
pub struct Feed {
    id: i32,
    feed_title: String,
    image_handle: Option<Handle>,
}

#[derive(Debug, Clone)]
pub enum Message {
    UnfollowFeed(i32),
    ViewEpisodesForShow(i32),
}

impl Feed {
    pub fn new(id: i32, feed_title: String, image_handle: Option<Handle>) -> Self {
        Self {
            id,
            feed_title,
            image_handle,
        }
    }
    pub fn view(&self) -> Element<Message> {
        if let Some(handle) = &self.image_handle {
            container(row!(
                image(handle).height(50),
                text(self.feed_title.to_owned()).width(Length::FillPortion(6)),
                button(text("Unfollow"))
                    .on_press(Message::UnfollowFeed(self.id))
                    .width(Length::FillPortion(3)),
                button(text("View Episodes"))
                    .on_press(Message::ViewEpisodesForShow(self.id))
                    .width(Length::FillPortion(3))
            ))
            .max_width(600)
            .padding(20)
            .center_x(Length::Shrink)
            // .center_y(Length::Fill)
            .into()
        } else {
            container(row!(
                text(self.feed_title.to_owned()).width(Length::FillPortion(6)),
                button(text("Unfollow"))
                    .on_press(Message::UnfollowFeed(self.id))
                    .width(Length::FillPortion(3)),
                button(text("View Episodes"))
                    .on_press(Message::ViewEpisodesForShow(self.id))
                    .width(Length::FillPortion(3))
            ))
            .max_width(600)
            .padding(20)
            .center_x(Length::Shrink)
            // .center_y(Length::Fill)
            .into()
        }
    }
}
