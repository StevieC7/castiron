use sqlite::open;
use std::path::Path;
use tokio::fs::create_dir;

use crate::types::errors::CustomError;

pub async fn create_shows_directory_if_not_existing() -> Result<(), CustomError> {
    match Path::new("./shows").exists() {
        true => Ok(()),
        false => {
            create_dir(Path::new("./shows")).await?;
            Ok(())
        }
    }
}

pub async fn create_episodes_directory_if_not_existing() -> Result<(), CustomError> {
    match Path::new("./episodes").exists() {
        true => Ok(()),
        false => {
            create_dir(Path::new("./episodes")).await?;
            Ok(())
        }
    }
}

pub async fn create_thumbnails_directory_if_not_existing() -> Result<(), CustomError> {
    match Path::new("./thumbnails").exists() {
        true => Ok(()),
        false => {
            create_dir(Path::new("./thumbnails")).await?;
            Ok(())
        }
    }
}

pub async fn create_database_if_not_existing() -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("
        CREATE TABLE IF NOT EXISTS feeds(id INTEGER PRIMARY KEY, url TEXT NOT NULL, xml_file_path TEXT, feed_title TEXT, image_file_path TEXT);
        CREATE TABLE IF NOT EXISTS episodes(id INTEGER PRIMARY KEY, guid TEXT, title TEXT, date DATE, played BOOLEAN, played_seconds INTEGER, file_name TEXT, url TEXT, feed_id INTEGER, downloaded BOOLEAN);
        CREATE UNIQUE INDEX IF NOT EXISTS guid_feed_id ON episodes (guid,feed_id);
        ");
    connection.execute(query)?;
    Ok(())
}
