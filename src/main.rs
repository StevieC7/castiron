use std::{io, fs, path::Path };
use std::fs::File;
use std::io::prelude::*;
use reqwest::{self, get};
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
            // first, check if there is a file for user already
            let open_file: Option<File> = get_feed_list();
            match open_file {
                Some(mut file) => {
                    println!("What feed do you want to update?");
                    let mut input_url: String = String::new();
                    io::stdin()
                    .read_line(&mut input_url)
                    .expect("Failed to read input.");
                    // if file, append url to list of saved URLs
                    let feed_xml = get_request(input_url);
                    match feed_xml {
                        Ok(xml) => {
                            println!("{}", xml);
                            let write_result = file.write(xml.as_bytes());
                            match write_result {
                                Ok(ok) => println!("Wrote to file: {}", ok),
                                Err(e) => println!("Error writing to file: {}", e)
                            }
                        },
                        Err(e) => println!("Error getting feed: {}", e)
                    }   
                }
                None => ()
            }
            // TODO: put a function in to save URLs from user input into a file
        },
        "LIST" => println!("You picked LIST"),
        // TODO: put a function in to read URLS from a file
        _ => println!("You picked wrong.")
    }
}

#[tokio::main]
async fn get_request(url: String) -> Result<String, reqwest::Error> {
    let result = get(url).await?.text().await?;
    Ok(result)
}

fn get_feed_list() -> Option<File> {
    let path = Path::new( "./foo.txt");
    let file = fs::File::create(path);
    match file {
        Ok( file ) => {
            Some(file)
        },
        Err(e) => { 
            println!("Error creating file: {}", e);
            None
        }
    }
}