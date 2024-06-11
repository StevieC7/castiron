use std::path::Path;
use tokio::fs::create_dir;

pub async fn create_shows_directory_if_not_existing() -> Result<(), String> {
    match Path::new("./shows").exists() {
        true => Ok(()),
        false => match create_dir(Path::new("./shows")).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        },
    }
}

pub async fn create_episodes_directory_if_not_existing() -> Result<(), String> {
    match Path::new("./episodes").exists() {
        true => Ok(()),
        false => match create_dir(Path::new("./episodes")).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        },
    }
}
