use iced::{
    advanced::image::Handle,
    widget::{button, container, image, row, text},
    Element, Length, Task,
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
    ImageLoaded(i32, Option<Handle>),
}

impl Feed {
    pub fn new(
        id: i32,
        feed_title: String,
        image_file_path: Option<String>,
    ) -> (Self, Task<Message>) {
        (
            Self {
                id,
                feed_title,
                image_handle: None,
            },
            Task::perform(
                async {
                    match image_file_path {
                        Some(fp) => Some(Handle::from(fp)),
                        None => None,
                    }
                },
                move |handle| Message::ImageLoaded(id, handle),
            ),
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ImageLoaded(id, handle) => {
                if id == self.id {
                    match handle {
                        Some(image_handle) => {
                            self.image_handle = Some(image_handle);
                            Task::none()
                        }
                        None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::UnfollowFeed(_id) => Task::none(),
            Message::ViewEpisodesForShow(_id) => Task::none(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        let image = match &self.image_handle {
            Some(handle) => image(handle),
            None => image(""),
        };
        container(row!(
            image.height(50),
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
