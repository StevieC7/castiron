use iced::{
    alignment::Horizontal,
    time,
    widget::{
        button, container, progress_bar, progress_bar::Style, row, text, Renderer, Text, Theme,
    },
    Element, Length, Subscription,
};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::{fs::File, io::BufReader, time::Duration};

use crate::{file_handling::episodes::get_episode_by_id, ui::gui::Message};

#[allow(dead_code)] // Sink is the handle to the stream, but if stream is dropped, playback stops.
#[derive(Default)]
pub struct Player {
    pub id: Option<i32>,
    stream: Option<OutputStream>,
    pub sink: Option<Sink>,
    pub progress: f32,
    pub duration_seconds: f32,
}

#[derive(Clone, Debug)]
pub enum PlayerMessage {
    Play,
    Pause,
    Progress,
}

impl Player {
    pub fn new(id: Option<i32>) -> Self {
        match id {
            None => Self::default(),
            Some(id) => match get_episode_by_id(id) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    Self::default()
                }
                Ok(episode) => match OutputStreamBuilder::open_default_stream() {
                    Err(e) => {
                        eprintln!("{:?}", e);
                        Self::default()
                    }
                    Ok(stream_handle) => {
                        let sink = Sink::connect_new(&stream_handle.mixer());
                        match File::open(format!("./episodes/{}", episode.file_name)) {
                            Err(e) => {
                                eprintln!("{:?}", e);
                                Self::default()
                            }
                            Ok(file) => {
                                let file_meta = file.metadata();
                                let byte_len = match file_meta {
                                    Ok(meta) => meta.len(),
                                    Err(_) => 0,
                                };
                                let file_buf = BufReader::new(file);
                                match Decoder::builder()
                                    .with_data(file_buf)
                                    .with_byte_len(byte_len)
                                    .build()
                                {
                                    Err(e) => {
                                        eprintln!("{:?}", e);
                                        Self::default()
                                    }
                                    Ok(source) => {
                                        let duration_secs = match source.total_duration() {
                                            Some(dur) => dur.as_secs_f32(),
                                            None => 0.0,
                                        };
                                        sink.append(source);
                                        sink.play();
                                        Self {
                                            id: Some(episode.id),
                                            stream: Some(stream_handle),
                                            sink: Some(sink),
                                            progress: 0.0,
                                            duration_seconds: duration_secs,
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            },
        }
    }

    pub fn update(&mut self, message: PlayerMessage) {
        match message {
            PlayerMessage::Play => match &self.sink {
                Some(sink) => sink.play(),
                None => (),
            },
            PlayerMessage::Pause => match &self.sink {
                Some(sink) => sink.pause(),
                None => (),
            },
            PlayerMessage::Progress => match &self.sink {
                Some(sink) => match sink.empty() {
                    true => {
                        self.progress = 0.0;
                    }
                    false => {
                        let position = sink.get_pos();
                        self.progress = position.as_secs_f32();
                    }
                },
                None => (),
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![time::every(Duration::from_millis(100))
            .map(|_| Message::PlayerMessage(PlayerMessage::Progress))])
    }

    pub fn view(&self) -> Element<Message> {
        let title: Text<Theme, Renderer> = match self.id {
            Some(id) => {
                let episode = get_episode_by_id(id);
                match episode {
                    Ok(episode) => text(format!("{}", episode.title)),
                    Err(_) => text("Not Playing"),
                }
            }
            None => text("Not Playing"),
        };
        container(
            row!(
                title,
                button(text("Play")).on_press(Message::PlayerMessage(PlayerMessage::Play)),
                button(text("Pause")).on_press(Message::PlayerMessage(PlayerMessage::Pause)),
                progress_bar(0.0..=self.duration_seconds, self.progress).style(|theme: &Theme| {
                    let foo = theme.palette();
                    Style {
                        background: iced::Background::Color(foo.text),
                        bar: iced::Background::Color(foo.success),
                        border: iced::Border::default(),
                    }
                })
            )
            .spacing(10),
        )
        .width(Length::Fill)
        .padding(20)
        .align_x(Horizontal::Center)
        .into()
    }
}
