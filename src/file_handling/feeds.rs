use crate::types::{errors::CustomError, feeds::FeedMeta};
use sqlite::open;
use std::{
    fs::{read_to_string, remove_file},
    io::Error as IOError,
    path::Path,
};

pub fn add_feed_to_database(url: String) -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("CREATE TABLE IF NOT EXISTS feeds(id INTEGER PRIMARY KEY, url TEXT NOT NULL, xml_file_path TEXT, feed_title TEXT); INSERT INTO feeds (url,xml_file_path,feed_title) VALUES ('{url}', NULL, NULL);");
    connection.execute(query)?;
    Ok(())
}

pub fn update_feed_title(id: i32, title: String) -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let mut sanitized_title = title.replace("'", "''");
    sanitized_title = sanitized_title.replace("\"", "\"\"");
    let query = format!("UPDATE feeds SET feed_title = '{sanitized_title}' WHERE id = {id};");
    connection.execute(query)?;
    Ok(())
}

pub fn update_feed_file_path(id: i32, file_path: String) -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("UPDATE feeds SET xml_file_path = '{file_path}' WHERE id = {id};");
    connection.execute(query)?;
    Ok(())
}

pub fn get_feed_list_database() -> Result<Vec<FeedMeta>, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = "SELECT * FROM feeds";
    let mut feeds: Vec<FeedMeta> = Vec::new();
    connection.iterate(query, |n| {
        let mut result_tuple: FeedMeta = FeedMeta {
            id: 0,
            feed_url: String::new(),
            xml_file_path: None,
            feed_title: None,
        };
        let id_kv_tuple = n.iter().find(|val| val.0 == "id");
        match id_kv_tuple {
            Some(wrapped_id) => match wrapped_id.1 {
                Some(id) => result_tuple.id = id.to_string().parse().unwrap(),
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
        let title_kv_pair = n.iter().find(|val| val.0 == "feed_title");
        match title_kv_pair {
            Some(title_tuple) => match title_tuple.1 {
                Some(title) => {
                    result_tuple.feed_title = Some(title.to_string());
                }
                None => (),
            },
            None => (),
        }
        feeds.push(result_tuple);
        true
    })?;
    Ok(feeds)
}

pub fn get_feed_by_id(id: i32) -> Result<FeedMeta, CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("SELECT * FROM feeds WHERE id = {id} LIMIT 1;");
    let mut result_tuple: FeedMeta = FeedMeta {
        id: 0,
        feed_url: String::new(),
        xml_file_path: None,
        feed_title: None,
    };
    connection.iterate(query, |n| {
        let id_kv_tuple = n.iter().find(|val| val.0 == "id");
        match id_kv_tuple {
            Some(wrapped_id) => match wrapped_id.1 {
                Some(id) => result_tuple.id = id.to_string().parse().unwrap(),
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
        let title_kv_pair = n.iter().find(|val| val.0 == "feed_title");
        match title_kv_pair {
            Some(title_tuple) => match title_tuple.1 {
                Some(title) => {
                    result_tuple.feed_title = Some(title.to_string());
                }
                None => (),
            },
            None => (),
        }
        true
    })?;
    Ok(result_tuple)
}

pub fn delete_associated_episodes_and_xml(id: i32) -> Result<(), CustomError> {
    let connection = open(Path::new("./database.sqlite"))?;
    let query = format!("SELECT xml_file_path FROM feeds WHERE id = {id};");
    let mut xml_file_path = String::new();
    connection.iterate(query, |n| {
        match n.iter().nth(0) {
            Some(wrapped_xml_file_path) => match wrapped_xml_file_path.1 {
                Some(file_path) => xml_file_path = file_path.to_string(),
                None => (),
            },
            None => (),
        }
        true
    })?;
    remove_file(Path::new(format!("{xml_file_path}").as_str()))?;
    let query = format!("DELETE FROM feeds WHERE id = {id};");
    connection.execute(query)?;
    let query = format!("DELETE FROM episodes WHERE feed_id = {id} RETURNING file_name;");
    connection.iterate(query, |row| {
        match row.iter().nth(0) {
            Some(wrapped_file_name) => match wrapped_file_name.1 {
                Some(file_name) => {
                    remove_file(Path::new(format!("./episodes/{file_name}").as_str())).unwrap_or(())
                }
                None => (),
            },
            None => (),
        };
        true
    })?;
    Ok(())
}

pub fn load_feed_xml(xml_file_path: String) -> Result<String, IOError> {
    let data = read_to_string(xml_file_path)?;
    Ok(data)
}

#[cfg(test)]
mod tests {

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
}
