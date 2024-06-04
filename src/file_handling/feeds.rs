use crate::types::{errors::CustomError, feeds::FeedMeta};
use sqlite::open;
use std::{
    fs::{read_dir, read_to_string},
    io::Error as IOError,
    path::Path,
};

pub fn add_feed_to_database(url: String) -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("CREATE TABLE IF NOT EXISTS feeds(id INTEGER PRIMARY KEY, url TEXT NOT NULL, xml_file_path TEXT); INSERT INTO feeds (url,xml_file_path) VALUES ('{url}', NULL);");
    connection.execute(query)?;
    Ok(())
}

pub fn get_feed_list_database() -> Result<Vec<FeedMeta>, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = "SELECT * FROM feeds";
    let mut feeds: Vec<FeedMeta> = Vec::new();
    connection.iterate(query, |n| {
        let mut result_tuple: FeedMeta = FeedMeta {
            index: 0,
            feed_url: String::new(),
            xml_file_path: None,
        };
        let id_kv_tuple = n.iter().find(|val| val.0 == "id");
        match id_kv_tuple {
            Some(wrapped_id) => match wrapped_id.1 {
                Some(id) => result_tuple.index = id.to_string().parse().unwrap(),
                None => (),
            },
            None => (),
        }
        let url_kv_tuple = n.iter().find(|val| val.0 == "url");
        match url_kv_tuple {
            Some(wrapped_url) => match wrapped_url.1 {
                Some(url) => result_tuple.feed_url = url.to_string(),
                None => (),
            },
            None => (),
        }
        let xml_kv_tuple = n.iter().find(|val| val.0 == "xml_file_path");
        match xml_kv_tuple {
            Some(wrapped_xml) => match wrapped_xml.1 {
                Some(xml) => result_tuple.xml_file_path = Some(xml.to_string()),
                None => (),
            },
            None => (),
        }
        feeds.push(result_tuple);
        true
    })?;
    Ok(feeds)
}

pub fn load_feeds_xml() -> Result<Vec<String>, IOError> {
    println!("Loading feeds xml");
    let feed_path_list = read_dir("./shows")?;
    let mut feed_collection = Vec::new();
    for feed in feed_path_list {
        match feed {
            Ok(directory) => {
                if let Some("./shows/.DS_Store") = directory.path().to_str() {
                    continue;
                }
                let feed_content = read_to_string(directory.path())?;
                feed_collection.push(feed_content);
            }
            Err(e) => println!("{e}"),
        }
    }
    Ok(feed_collection)
}

pub fn check_episode_exists(file_name: &str) -> Result<bool, IOError> {
    let mut episode_list = read_dir("./episodes")?;
    let found_existing = episode_list.find(|episode| match episode {
        Ok(entry) => {
            if entry.file_name() == file_name {
                true
            } else {
                false
            }
        }
        Err(_e) => false,
    });
    match found_existing {
        Some(_thing) => Ok(true),
        None => Ok(false),
    }
}

pub fn get_feed_id_by_url(url: &String) -> Result<i32, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("SELECT id FROM feeds WHERE url = '{url}' LIMIT 1;");
    let mut result = 0;
    connection.iterate(query, |n| {
        let id_kv_tuple = n.iter().find(|val| val.0 == "id");
        match id_kv_tuple {
            Some(wrapped_id) => match wrapped_id.1 {
                Some(id) => result = id.to_string().parse().unwrap(),
                None => (),
            },
            None => (),
        }
        true
    })?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn test_add_feed() {
        let url = String::from("https://www.google.com");
        assert!(add_feed_to_database(url).is_ok())
    }

    #[test]
    fn test_get_feed_list() {
        if open(Path::new("./database.sqlite")).is_ok() {
            assert!(get_feed_list_database().is_ok())
        } else {
            assert!(get_feed_list_database().is_err())
        }
    }

    #[test]
    fn test_load_feeds_xml() {
        let shows_directory = read_dir("./shows");
        match shows_directory {
            Ok(mut result) => {
                let first = match result.nth(0) {
                    Some(dir_entry) => match dir_entry {
                        Ok(val) => Some(val.file_name()),
                        Err(_) => None,
                    },
                    None => None,
                };
                // Check for OSX specific DS_Store file
                if first != Some(OsString::from(".DS_Store")) && result.count() > 0 {
                    assert!(load_feeds_xml().is_ok())
                } else {
                    assert!(load_feeds_xml().is_err())
                }
            }
            Err(_) => assert!(load_feeds_xml().is_err()),
        }
    }

    #[test]
    fn test_check_episode_exists() {
        if Path::new("./episodes").exists() {
            assert!(check_episode_exists("foo").is_ok())
        } else {
            assert!(check_episode_exists("foo").is_err())
        }
    }
}
