use rand::Rng;
pub fn update_feeds(feeds: Vec<FeedMeta>) {
    for feed in feeds {
        let updated_feed: Result<String, reqwest::Error> = get_request(&feed.feed_url);
        match updated_feed {
            Ok(_val) => {
                println!("Fetched feed: {:?}", feed.feed_url);
                let mut path_string: String =
                    format!("./shows/{:?}", rand::thread_rng().gen_range(0..10000));
                let path_exists = check_feed_exists(path_string.clone());
                match path_exists {
                    Ok(exists) => {
                        if exists {
                            println!("Path already exists, so we better generate a new one.")
                            // TODO: write the directory name to the list of feeds to reference later
                        } else {
                            println!("Path does not exist, so carry on as usual.");
                            let dir_path: &Path = Path::new(path_string.as_str());
                            let created_dir: Result<(), io::Error> = create_dir_all(dir_path);
                            match created_dir {
                                Ok(_) => {
                                    path_string.push_str("/feed.xml");
                                    let file_path = Path::new(path_string.as_str());
                                    let xml_file =
                                        OpenOptions::new().create(true).write(true).open(file_path);
                                    match xml_file {
                                        Ok(mut file) => {
                                            let seek_result: Result<u64, io::Error> =
                                                file.seek(io::SeekFrom::Start(0));
                                            match seek_result {
                                                Ok(_) => {
                                                    let result = file.write_all(_val.as_bytes());
                                                    // TODO: write the directory name to the list of feeds to reference later
                                                    match result {
                                                    Ok(_) => println!("Successfully created xml file for show: {:?}", feed.feed_url),
                                                    Err(e) => println!("Error writing fetched data to xml file: {e}")
                                                }
                                                }
                                                Err(e) => {
                                                    println!("Error seeking for write head: {e}")
                                                }
                                            }
                                        }
                                        Err(e) => println!("Error creating xml file: {e}"),
                                    }
                                }
                                Err(e) => println!("Error creating directory: {e}"),
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
            Err(e) => println!("Error fetching feed: {e}"),
        }
    }
}
