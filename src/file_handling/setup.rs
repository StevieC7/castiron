use sqlite::open;
use std::{fs::create_dir, path::Path};
// use tokio::fs::create_dir;

use crate::{
    file_handling::{
        config::load_or_create_config, episodes::get_episode_list_database,
        feeds::get_feed_list_database, queue::get_queue_database,
    },
    types::{config::CastironConfig, episodes::Episode, errors::CustomError, feeds::FeedMeta},
};

fn create_shows_directory_if_not_existing() -> Result<(), CustomError> {
    match Path::new("./shows").exists() {
        true => Ok(()),
        false => {
            create_dir(Path::new("./shows"))?;
            Ok(())
        }
    }
}

fn create_episodes_directory_if_not_existing() -> Result<(), CustomError> {
    match Path::new("./episodes").exists() {
        true => Ok(()),
        false => {
            create_dir(Path::new("./episodes"))?;
            Ok(())
        }
    }
}

fn create_thumbnails_directory_if_not_existing() -> Result<(), CustomError> {
    match Path::new("./thumbnails").exists() {
        true => Ok(()),
        false => {
            create_dir(Path::new("./thumbnails"))?;
            Ok(())
        }
    }
}

fn create_database_if_not_existing() -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("
        CREATE TABLE IF NOT EXISTS feeds(id INTEGER PRIMARY KEY, url TEXT NOT NULL, xml_file_path TEXT, feed_title TEXT, image_file_path TEXT);
        CREATE TABLE IF NOT EXISTS episodes(id INTEGER PRIMARY KEY, guid TEXT, title TEXT, date DATE, played BOOLEAN, played_seconds INTEGER, file_name TEXT, url TEXT, feed_id INTEGER, downloaded BOOLEAN);
        CREATE UNIQUE INDEX IF NOT EXISTS guid_feed_id ON episodes (guid,feed_id);
        CREATE TABLE IF NOT EXISTS queue(id INTEGER PRIMARY KEY, episodes JSON);
        ");
    connection.execute(query)?;
    Ok(())
}

fn load_existing_user_state() -> Result<InitData, CustomError> {
    let config = load_or_create_config()?;
    let feeds = get_feed_list_database()?;
    let episodes = get_episode_list_database()?;
    let queue = get_queue_database()?;
    Ok(InitData {
        config,
        feeds,
        episodes,
        queue,
    })
}
#[derive(Debug, Clone)]
pub struct InitData {
    pub config: CastironConfig,
    pub feeds: Vec<FeedMeta>,
    pub episodes: Vec<Episode>,
    pub queue: Vec<Episode>,
}
pub async fn init_fs_and_db() -> Result<InitData, CustomError> {
    create_shows_directory_if_not_existing()?;
    create_episodes_directory_if_not_existing()?;
    create_thumbnails_directory_if_not_existing()?;
    create_database_if_not_existing()?;
    let init_data = load_existing_user_state()?;
    Ok(init_data)
}
