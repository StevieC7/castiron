use iced::{
    advanced::image::Handle,
    widget::{button, container, image, row, text},
    Element, Length, Task,
};

#[derive(Debug, Clone)]
pub enum ImageState {
    Loading,
    Loaded(Option<Handle>),
}

#[derive(Debug, Clone)]
pub struct Feed {
    id: i32,
    feed_title: String,
    image_file_path: Option<String>,
    image_handle: Option<Handle>,
    image_state: ImageState,
}

#[derive(Debug, Clone)]
pub enum Message {
    UnfollowFeed(i32),
    ViewEpisodesForShow(i32),
    ImageState(ImageState),
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
                image_file_path,
                image_handle: None,
                image_state: ImageState::Loading,
            },
            Task::perform(async { todo!() }, |handle| {
                Message::ImageState(ImageState::Loaded(handle))
            }),
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        // match feed_message {
        //     FeedMessage::UnfollowFeed(id) => match delete_associated_episodes_and_xml(id) {
        //         Ok(_) => Task::perform(load_feeds(), Message::FeedsLoaded),
        //         Err(e) => {
        //             eprintln!("Error deleting feed: {:?}", e);
        //             Task::none()
        //         }
        //     },
        //     FeedMessage::ViewEpisodesForShow(_id) => Task::none(),
        //     FeedMessage::ImageState(_state) => Task::none(),
        // }
        match message {
            Message::ImageState(image_state) => match image_state {
                ImageState::Loading => Task::none(),
                ImageState::Loaded(handle) => match handle {
                    Some(image_handle) => {
                        self.image_handle = Some(image_handle);
                        Task::none()
                    }
                    None => Task::none(),
                },
            },
            _ => Task::none(),
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
