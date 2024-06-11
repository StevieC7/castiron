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

pub fn convert_theme_string_to_enum(theme: String) -> Theme {
    if theme == Theme::Light.to_string() {
        Theme::Light
    } else if theme == Theme::Dark.to_string() {
        Theme::Dark
    } else if theme == Theme::Dracula.to_string() {
        Theme::Dracula
    } else if theme == Theme::Nord.to_string() {
        Theme::Nord
    } else if theme == Theme::SolarizedLight.to_string() {
        Theme::SolarizedLight
    } else if theme == Theme::SolarizedDark.to_string() {
        Theme::SolarizedDark
    } else if theme == Theme::GruvboxLight.to_string() {
        Theme::GruvboxLight
    } else if theme == Theme::GruvboxDark.to_string() {
        Theme::GruvboxDark
    } else if theme == Theme::CatppuccinLatte.to_string() {
        Theme::CatppuccinLatte
    } else if theme == Theme::CatppuccinFrappe.to_string() {
        Theme::CatppuccinFrappe
    } else if theme == Theme::CatppuccinMacchiato.to_string() {
        Theme::CatppuccinMacchiato
    } else if theme == Theme::CatppuccinMocha.to_string() {
        Theme::CatppuccinMocha
    } else if theme == Theme::TokyoNight.to_string() {
        Theme::TokyoNight
    } else if theme == Theme::TokyoNightStorm.to_string() {
        Theme::TokyoNightStorm
    } else if theme == Theme::TokyoNightLight.to_string() {
        Theme::TokyoNightLight
    } else if theme == Theme::KanagawaWave.to_string() {
        Theme::KanagawaWave
    } else if theme == Theme::KanagawaDragon.to_string() {
        Theme::KanagawaDragon
    } else if theme == Theme::KanagawaLotus.to_string() {
        Theme::KanagawaLotus
    } else if theme == Theme::Moonfly.to_string() {
        Theme::Moonfly
    } else if theme == Theme::Nightfly.to_string() {
        Theme::Nightfly
    } else if theme == Theme::Oxocarbon.to_string() {
        Theme::Oxocarbon
    } else {
        Theme::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_config() {
        let test_config_value = CastironConfig {
            theme: Theme::default().to_string(),
        };
        assert!(create_config(Some(test_config_value)).is_ok())
    }

    #[test]
    fn test_read_config() {
        assert!(read_config().is_ok())
    }
}
