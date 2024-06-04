use crate::types::{episodes::Episode, errors::CustomError};
use sqlite::open;
use std::path::Path;

pub fn add_episode_to_database(episode: Episode) -> Result<(), CustomError> {
    let Episode {
        guid,
        title,
        date,
        file_path,
        url,
        feed_id,
        ..
    } = episode;
    let existing_file_path = match file_path {
        Some(val) => String::from(format!("'{val}'")),
        None => String::from("NULL"),
    };
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("
        CREATE TABLE IF NOT EXISTS episodes(guid TEXT PRIMARY KEY, title TEXT, date DATE, played BOOLEAN, played_seconds INTEGER, file_path TEXT, url TEXT, feed_id INTEGER);
        INSERT INTO episodes (guid, title, date, played, file_path, url, feed_id) VALUES ('{guid}', '{title}', '{date}', FALSE, {existing_file_path}, '{url}', '{feed_id}');
    ");
    connection.execute(query)?;
    Ok(())
}

pub fn get_episode_list_database() -> Result<Vec<Episode>, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = "SELECT * FROM episodes";
    let mut episodes: Vec<Episode> = Vec::new();
    connection.iterate(query, |n| {
        let mut result_tuple: Episode = Episode {
            guid: String::new(),
            title: String::new(),
            date: String::new(),
            played: false,
            played_seconds: 0,
            file_path: None,
            url: String::new(),
            feed_id: 0,
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
        let file_path_kv_tuple = n.iter().find(|val| val.0 == "file_path");
        match file_path_kv_tuple {
            Some(wrapped_file_path) => match wrapped_file_path.1 {
                Some(file_path) => result_tuple.file_path = Some(file_path.to_string()),
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
        episodes.push(result_tuple);
        true
    })?;
    Ok(episodes)
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
                    file_path: None,
                    played: false,
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
                file_path: None,
                played: false,
            })
            .is_ok())
        }
    }

    #[test]
    fn test_get_episode_list() {
        assert!(get_episode_list_database().is_ok())
    }
}
