use crate::networking::downloads::download_episode_by_guid;
use crate::ui::gui::{AppView, Message, PodQueueMessage};
use iced::{
    advanced::image::Handle,
    widget::{button, container, horizontal_space, image, row, text, Row},
    Element, Length, Renderer, Theme,
};

// TODO: implement download progress indicator
// TODO: look into optimizing image handling
pub struct Episode {
    pub id: i32,
    pub feed_id: i32,
    pub guid: String,
    pub title: String,
    pub downloaded: bool,
    pub viewing_from: AppView,
    pub image_handle: Option<Handle>,
}

impl Episode {
    pub fn new(
        id: i32,
        feed_id: i32,
        guid: String,
        title: String,
        downloaded: bool,
        viewing_from: AppView,
        image_handle: Option<Handle>,
    ) -> Self {
        Self {
            id,
            feed_id,
            guid,
            title,
            downloaded,
            viewing_from,
            image_handle,
        }
    }
    pub fn view(&self) -> Element<Message> {
        let action_container: Row<Message, Theme, Renderer> = match self.downloaded {
            true => match self.viewing_from {
                AppView::Queue => {
                    row!(button(text("Play")).on_press(Message::PlayEpisode(self.id)))
                }
                _ => row!(
                    button(text("Delete"))
                        .on_press(Message::DeleteEpisode(self.id))
                        .width(Length::FillPortion(3)),
                    horizontal_space().width(Length::FillPortion(1)),
                    button(text("Queue"))
                        .on_press(Message::PodQueueMessage(PodQueueMessage::AddToQueue(
                            self.id
                        )))
                        .width(Length::FillPortion(3)),
                    button(text("Play"))
                        .on_press(Message::PlayEpisode(self.id))
                        .width(Length::FillPortion(3))
                ),
            },
            false => row!(button(text("Download")).on_press(Message::DownloadEpisode(self.id)),),
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

    pub async fn download_single_episode(id: i32) -> Result<(), String> {
        match download_episode_by_guid(id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from(format!("Error downloading episode: {:?}", e))),
        }
    }
}
