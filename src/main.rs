use reqwest::{self, get};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::{fs, io, path::Path};
use tokio;
fn main() {
    // TODO: make sure the function for fetching and saving from saved URLs runs on launch
    println!("ADD podcast or LIST shows?");
    let mut mode_selection: String = String::new();
    io::stdin()
        .read_line(&mut mode_selection)
        .expect("Failed to read input.");
    match mode_selection.as_str().trim() {
        "ADD" => {
            println!("You picked ADD");
            let open_file: Option<File> = get_feed_list();
            match open_file {
                Some(file) => {
                    println!("What feed do you want to follow?");
                    let mut input_url: String = String::new();
                    io::stdin()
                        .read_line(&mut input_url)
                        .expect("Failed to read input.");
                    let feed_result: Option<File> = add_feed_to_list(input_url, file);
                    match feed_result {
                        Some( _file ) => {
                            let contents = fs::read_to_string(Path::new("./feed_list.txt")).expect("Oopsie reading saved file");
                            println!("{contents}");
                        },
                        None => println!("Error saving feed to list.")
                    }
                }
                None => (),
            }
            // TODO: put a function in to save URLs from user input into a file
        }
        "LIST" => println!("You picked LIST"),
        // TODO: put a function in to read URLS from a file
        _ => println!("You picked wrong."),
    }
}

#[tokio::main]
async fn get_request(url: String) -> Result<String, reqwest::Error> {
    let result = get(url).await?.text().await?;
    Ok(result)
}

fn get_feed_list() -> Option<File> {
    let path = Path::new("./feed_list.txt");
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path);
    match file {
        Ok(file) => Some(file),
        Err(e) => {
            println!("Error finding feed list file: {}", e);
            None
        }
    }
}

fn add_feed_to_list(url: String, mut file: File) -> Option<File> {
    let mut trimmed_url = url.trim().to_string();
    trimmed_url.insert_str(0, "{\"feed_url\":\"");
    trimmed_url.push_str("\"},\n");
    let result: Result<(), io::Error> = file.write_all(trimmed_url.as_bytes());
    match result {
        Ok(_val) => Some(file),
        Err(e) => {
            println!( "{}", e );
            None
        }
    }
}
