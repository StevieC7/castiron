use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use crate::types::config::CastironConfig;
use crate::types::errors::CustomError;
use serde_json::{from_reader, to_writer};

pub fn read_config() -> Result<CastironConfig, CustomError> {
    let config_file = File::open(Path::new("./castiron_config.json"))?;
    let config: CastironConfig = from_reader(BufReader::new(config_file))?;
    Ok(config)
}

pub fn create_config(config: CastironConfig) -> Result<(), CustomError> {
    let config_file_path = Path::new("./castiron_config.json");
    let config_file = File::create(config_file_path)?;
    let mut writer = BufWriter::new(config_file);
    to_writer(&mut writer, &config)?;
    writer.flush()?;
    Ok(())
}
