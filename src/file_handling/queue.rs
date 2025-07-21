use crate::types::{episodes::Episode, errors::CustomError};
use sqlite::{open, Error};
use std::path::Path;

// TODO: decide when to call this
pub fn save_queue(queue: Vec<i32>) -> Result<(), CustomError> {
    let queue_string = queue.iter().map(|q| q.to_string()).reduce(|mut acc, s| {
        acc.push_str(String::from(format!(",{id}", id = s)).as_str());
        acc
    });
    match queue_string {
        Some(q) => {
            let query = format!(
                "
                DELETE FROM queue;
                INSERT INTO queue (episodes) VALUES (json_array({q}));
            "
            );
            let connection = open(Path::new("./database.sqlite"))?;
            connection.execute(query)?;
            Ok(())
        }
        None => Err(CustomError::SqlError(Error {
            code: None,
            message: Some(String::from("Error saving queue.")),
        })),
    }
}

pub fn get_queue_database() -> Result<Vec<Episode>, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query =
        "SELECT * FROM episodes WHERE id IN (SELECT value FROM queue, json_each(queue.episodes));";
    let mut queue: Vec<Episode> = Vec::new();
    connection.iterate(query, |n| select_all_callback(n, &mut queue))?;
    Ok(queue)
}

fn select_all_callback(n: &[(&str, Option<&str>)], episodes: &mut Vec<Episode>) -> bool {
    println!("thing: {:?}", n);
    let mut result_tuple: Episode = Episode {
        id: 0,
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
    let id_kv_tuple = n.iter().find(|val| val.0 == "id");
    match id_kv_tuple {
        Some(wrapped_id) => match wrapped_id.1 {
            Some(id) => {
                result_tuple.id = match id.parse::<i32>() {
                    Ok(parsed) => parsed,
                    Err(_) => 0,
                }
            }
            None => (),
        },
        None => (),
    }
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
// TODO: implement a cleanup function to wipe previous queues
