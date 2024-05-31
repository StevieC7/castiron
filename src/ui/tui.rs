use async_recursion::async_recursion;
use std::io;

use crate::{
    file_handling::feeds::{add_feed_to_database, get_feed_list_database},
    networking::{downloads::download_episodes, feeds::update_feeds},
};

#[async_recursion]
pub async fn tui_loop() {
    let feed_list_result = update_feeds().await;
    match feed_list_result {
        Ok(_) => println!("Finished updating feeds."),
        Err(e) => println!("Error occurred while fetching feed list: {:?}", e),
    }
    let download_result = download_episodes().await;
    match download_result {
        Ok(_) => println!("Finished downloading episodes."),
        Err(e) => println!("Error occurred while downloading episodes: {:?}", e),
    }
    println!("ADD podcast, LIST shows");
    let mut mode_selection: String = String::new();
    io::stdin()
        .read_line(&mut mode_selection)
        .expect("Failed to read input.");
    match mode_selection.as_str().trim() {
        "ADD" => {
            println!("What feed do you want to follow?");
            let mut input_url: String = String::new();
            io::stdin()
                .read_line(&mut input_url)
                .expect("Failed to read input.");
            let result = add_feed_to_database(input_url);
            match result {
                Ok(_) => println!("Feed added to list."),
                Err(e) => println!("An error occured while adding feed to list: {:?}", e),
            }
        }
        "LIST" => {
            println!("------Feeds You Follow------");
            if let Ok(urls) = get_feed_list_database() {
                urls.iter()
                    .for_each(|n| println!("{}: {}", n.index, n.feed_url));
            } else {
                println!("something wrong")
            }
        }
        // TODO: put a function in to read URLS from an OPML file
        _ => {
            println!("You picked wrong.");
            tui_loop().await;
        }
    }
}
