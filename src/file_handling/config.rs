use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use crate::types::config::CastironConfig;
use crate::types::errors::CustomError;
use iced::Theme;
use serde_json::{from_reader, to_writer};

pub fn read_config() -> Result<CastironConfig, CustomError> {
    let config_file = File::open(Path::new("./castiron_config.json"))?;
    let config: CastironConfig = from_reader(BufReader::new(config_file))?;
    Ok(config)
}

pub fn create_config(config: Option<CastironConfig>) -> Result<CastironConfig, CustomError> {
    match config {
        Some(conf) => {
            let config_file_path = Path::new("./castiron_config.json");
            let config_file = File::create(config_file_path)?;
            let mut writer = BufWriter::new(config_file);
            to_writer(&mut writer, &conf)?;
            writer.flush()?;
            Ok(conf)
        }
        None => {
            let conf = CastironConfig {
                auto_dl_new: true,
                auto_rm_after_listen: true,
                theme: Theme::default().to_string(),
            };
            let config_file_path = Path::new("./castiron_config.json");
            let config_file = File::create(config_file_path)?;
            let mut writer = BufWriter::new(config_file);
            to_writer(&mut writer, &conf)?;
            writer.flush()?;
            Ok(conf)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_config() {
        let test_config_value = CastironConfig {
            auto_dl_new: false,
            auto_rm_after_listen: false,
            theme: Theme::default().to_string(),
        };
        assert!(create_config(Some(test_config_value)).is_ok())
    }

    #[test]
    fn test_read_config() {
        assert!(read_config().is_ok())
    }
}
