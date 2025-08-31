use iced::{
    advanced::image::Handle,
    widget::{button, container, horizontal_space, image, row, text, Row},
    Element, Length, Renderer, Task, Theme,
};

use crate::{
    file_handling::episodes::delete_episode_from_fs, networking::downloads::download_episode_by_id,
};

// TODO: implement download progress indicator
pub struct Episode {
    pub id: i32,
    pub feed_id: i32,
    pub guid: String,
    pub title: String,
    pub downloaded: bool,
    pub image_handle: Option<Handle>,
    pub show_queue_controls: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Delete,
    AddToQueue,
    Download,
    DownloadComplete,
}

impl Episode {
    pub fn new(
        id: i32,
        feed_id: i32,
        guid: String,
        title: String,
        downloaded: bool,
        image_handle: Option<Handle>,
        show_queue_controls: bool,
    ) -> Self {
        Self {
            id,
            feed_id,
            guid,
            title,
            downloaded,
            image_handle,
            show_queue_controls,
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Delete => {
                // TODO: handle failure properly
                match delete_episode_from_fs(self.id) {
                    Ok(_) => {
                        self.downloaded = false;
                        Task::none()
                    }
                    Err(_) => Task::none(),
                }
            }
            Message::Download => {
                Task::perform(download_episode_by_id(self.id), move |res| match res {
                    Ok(_) => Message::DownloadComplete,
                    Err(e) => {
                        eprintln!("ERROR: {:?}", e);
                        todo!()
                    }
                })
            }
            Message::DownloadComplete => {
                self.downloaded = true;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut action_container: Row<Message, Theme, Renderer> = match self.downloaded {
            true => row!(
                button(text("Delete"))
                    .on_press(Message::Delete)
                    .width(Length::FillPortion(3)),
                horizontal_space().width(Length::FillPortion(1)),
                button(text("Queue"))
                    .on_press(Message::AddToQueue)
                    .width(Length::FillPortion(3)),
                button(text("Play"))
                    .on_press(Message::Play)
                    .width(Length::FillPortion(3))
            ),
            false => row!(button(text("Download")).on_press(Message::Download)),
        };
        match self.show_queue_controls {
            true => {
                action_container = action_container.push(button(text("Move Up")));
            }
            false => {}
        };
        match &self.image_handle {
            Some(handle) => container(row!(
                image(handle).height(100),
                text(self.title.to_owned()).width(300),
                action_container
            ))
            .width(Length::Shrink)
            .max_width(600)
            .padding(20)
            .center_y(Length::Shrink)
            .into(),
            None => container(row!(
                text(self.title.to_owned()).width(300),
                action_container
            ))
            .width(Length::Shrink)
            .max_width(600)
            .padding(20)
            .center_y(Length::Shrink)
            .into(),
        }
    }
}
