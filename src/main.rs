mod file_handling;
mod networking;
mod types;
mod ui;

use iced::{application, Task};
use ui::gui::{Castiron, Message};

use crate::file_handling::setup::init_fs_and_db;

fn main() -> iced::Result {
    // TODO: fix styling / theming
    application("Castiron", Castiron::update, Castiron::view)
        .theme(Castiron::theme)
        .subscription(Castiron::subscription)
        .exit_on_close_request(false)
        .run_with(|| {
            (
                Castiron::default(),
                Task::perform(init_fs_and_db(), |res| match res {
                    Ok(_) => Message::InitComplete,
                    Err(_) => Message::InitFailed,
                }),
            )
        })
}
