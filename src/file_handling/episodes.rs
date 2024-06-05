use crate::types::{episodes::Episode, errors::CustomError};
use sqlite::{open, Error};
use std::path::Path;

pub fn add_episode_to_database(episode: Episode) -> Result<(), CustomError> {
    let Episode {
        guid,
        title,
        date,
        file_name,
        url,
        feed_id,
        ..
    } = episode;
    let mut sanitized_title = title.replace("'", "''");
    sanitized_title = sanitized_title.replace("\"", "\"\"");
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("
        CREATE TABLE IF NOT EXISTS episodes(guid TEXT PRIMARY KEY, title TEXT, date DATE, played BOOLEAN, played_seconds INTEGER, file_name TEXT, url TEXT, feed_id INTEGER, downloaded BOOLEAN);
        INSERT INTO episodes (guid, title, date, played, file_name, url, feed_id, downloaded) VALUES ('{guid}', '{sanitized_title}', '{date}', FALSE, '{file_name}', '{url}', '{feed_id}', FALSE)
            ON CONFLICT (guid) DO NOTHING;
    ");
    connection.execute(query)?;
    Ok(())
}

pub fn get_episode_list_database() -> Result<Vec<Episode>, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = "SELECT * FROM episodes";
    let mut episodes: Vec<Episode> = Vec::new();
    connection.iterate(query, |n| select_all_callback(n, &mut episodes))?;
    Ok(episodes)
}

pub fn get_episode_by_guid(guid: String) -> Result<Episode, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("SELECT * FROM episodes WHERE guid = '{guid}'");
    let mut episodes: Vec<Episode> = Vec::new();
    connection.iterate(query, |n| select_all_callback(n, &mut episodes))?;
    match episodes.is_empty() {
        true => Err(CustomError::SqlError(Error {
            code: None,
            message: Some(String::from("No episode found.")),
        })),
        false => Ok(episodes.remove(0)),
    }
}

pub fn update_episode_download_status(guid: String) -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("UPDATE episodes WHERE guid = '{guid}' SET downloaded = TRUE;");
    connection.execute(query)?;
    Ok(())
}

fn select_all_callback(n: &[(&str, Option<&str>)], episodes: &mut Vec<Episode>) -> bool {
    let mut result_tuple: Episode = Episode {
        guid: String::new(),
        title: String::new(),
        date: String::new(),
        played: false,
        played_seconds: 0,
        file_name: String::new(),
        url: String::new(),
        feed_id: 0,
        downloaded: false,
    };
    let guid_kv_tuple = n.iter().find(|val| val.0 == "guid");
    match guid_kv_tuple {
        Some(wrapped_guid) => match wrapped_guid.1 {
            Some(guid) => result_tuple.guid = guid.to_string(),
            None => (),
        },
        None => (),
    }
    let title_kv_tuple = n.iter().find(|val| val.0 == "title");
    match title_kv_tuple {
        Some(wrapped_title) => match wrapped_title.1 {
            Some(title) => result_tuple.title = title.to_string(),
            None => (),
        },
        None => (),
    }
    let date_kv_tuple = n.iter().find(|val| val.0 == "date");
    match date_kv_tuple {
        Some(wrapped_date) => match wrapped_date.1 {
            Some(date) => result_tuple.date = date.to_string(),
            None => (),
        },
        None => (),
    }
    let played_kv_tuple = n.iter().find(|val| val.0 == "played");
    match played_kv_tuple {
        Some(wrapped_played) => match wrapped_played.1 {
            Some(played) => {
                result_tuple.played = match played.parse::<i8>() {
                    Ok(parsed) => match parsed {
                        0 => false,
                        1 => true,
                        _ => false,
                    },
                    Err(_) => false,
                }
            }
            None => (),
        },
        None => (),
    }
    let played_seconds_kv_tuple = n.iter().find(|val| val.0 == "played_seconds");
    match played_seconds_kv_tuple {
        Some(wrapped_played_seconds) => match wrapped_played_seconds.1 {
            Some(played_seconds) => {
                result_tuple.played_seconds = played_seconds.parse::<i32>().unwrap()
            }
            None => (),
        },
        None => (),
    }
    let file_path_kv_tuple = n.iter().find(|val| val.0 == "file_name");
    match file_path_kv_tuple {
        Some(wrapped_file_path) => match wrapped_file_path.1 {
            Some(file_path) => result_tuple.file_name = file_path.to_string(),
            None => (),
        },
        None => (),
    }
    let url_kv_tuple = n.iter().find(|val| val.0 == "url");
    match url_kv_tuple {
        Some(wrapped_url) => match wrapped_url.1 {
            Some(url) => result_tuple.url = url.to_string(),
            None => (),
        },
        None => (),
    }
    let feed_id_kv_tuple = n.iter().find(|val| val.0 == "feed_id");
    match feed_id_kv_tuple {
        Some(wrapped_feed_id) => match wrapped_feed_id.1 {
            Some(feed_id) => result_tuple.feed_id = feed_id.parse::<i32>().unwrap(),
            None => (),
        },
        None => (),
    }
    let downloaded_kv_tuple = n.iter().find(|val| val.0 == "downloaded");
    match downloaded_kv_tuple {
        Some(wrapped_downloaded) => match wrapped_downloaded.1 {
            Some(downloaded) => {
                result_tuple.downloaded = match downloaded.parse::<i8>() {
                    Ok(parsed) => match parsed {
                        0 => false,
                        1 => true,
                        _ => false,
                    },
                    Err(_) => false,
                }
            }
            None => (),
        },
        None => (),
    }
    episodes.push(result_tuple);
    true
}

#[cfg(test)]
mod tests {
    use std::fs::{copy, remove_file};

    use super::*;

    #[test]
    fn test_add_episode() {
        if Path::new("./database.sqlite").exists() {
            let existing_db_file_path = Path::new("./database.sqlite");
            let new_db_file = Path::new("./temp_db.sqlite");
            let copy_result = copy(&existing_db_file_path, &new_db_file).is_ok();
            let delete_old_result = remove_file(&existing_db_file_path).is_ok();
            if let true = copy_result & delete_old_result {
                assert!(add_episode_to_database(Episode {
                    date: String::from("2024/05/30"),
                    guid: String::from("jkdfjskluizuien1"),
                    title: String::from("Interesting Show Title"),
                    url: String::from("https://www.google.com"),
                    feed_id: 998,
                    played_seconds: 0,
                    file_name: String::from("pod.mp3"),
                    played: false,
                    downloaded: false
                })
                .is_ok());
                if let true = copy(&new_db_file, &existing_db_file_path).is_ok() {
                    let result = remove_file(&new_db_file);
                    match result {
                        Ok(_) => assert!(true),
                        Err(_) => panic!("Test failed due to test internals."),
                    };
                } else {
                    panic!("Test failed due to test internals.")
                }
            } else {
                panic!("Test failed due to test internals.")
            };
        } else {
            assert!(add_episode_to_database(Episode {
                date: String::from("2024/05/30"),
                guid: String::from("jkdfjskluizuien1"),
                title: String::from("Interesting Show Title"),
                url: String::from("https://www.google.com"),
                feed_id: 999,
                played_seconds: 0,
                file_name: String::from("pod.mp3"),
                played: false,
                downloaded: false,
            })
            .is_ok())
        }
    }

    #[test]
    fn test_get_episode_list() {
        assert!(get_episode_list_database().is_ok())
    }
}
