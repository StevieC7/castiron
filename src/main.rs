use std::{io, fs, path::Path };
use std::fs::File;
use std::io::prelude::*;
use reqwest::{self, get};
use tokio;
fn main() {
    // TODO: put in a function for asking user which mode they want (add new, fetch existing)
    // TODO: put a function in to save URLs from user input into a file
    // first, check if there is a file for user already
    let open_file: Option<File> = None;
    get_feed_list();
    match open_file {
        Some(file) => println!("feed list found {:?}", file),
        None => ()
    }
    println!("What feed do you want to update?");
    let mut input_url: String = String::new();
    io::stdin()
    .read_line(&mut input_url)
    .expect("Failed to read input.");
    // if file, append url to list of saved URLs
    let feed_xml = get_request(input_url);
    match feed_xml {
        Ok(xml) => println!("{}", xml),
        Err(e) => println!("{}", e)
    }   
    // TODO: put a function in to read URLS from a file
    // TODO: make sure the function for fetching and saving from saved URLs runs on launch
    // TODO: put a function in to save text retrieved from URL
}

#[tokio::main]
async fn get_request(url: String) -> Result<String, reqwest::Error> {
    let result = get(url).await?.text().await?;
    Ok(result)
}

fn create_feed_list(path: &Path) -> std::io::Result<File> {
    let mut file = File::create(path)?;
    file.write_all(b"Hello, world!")?;
    Ok(file)
}

fn get_feed_list() -> Option<File> {
    let path = Path::new( "./foo.txt");
    let existing_file = fs::File::open(path);
    match existing_file {
        Ok( file ) => {
            println!("file exists {:?}", file);
            None
        },
        _ => {
            // if no file, create one
            match create_feed_list(path) {
                Ok(file) => Some(file),
                _ => None
            }
        }
    }
}