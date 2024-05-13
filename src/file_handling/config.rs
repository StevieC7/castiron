use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::types::config::CastironConfig;
use crate::types::errors::CustomError;
use serde_json::to_writer;

pub fn read_config() -> Result<CastironConfig, CustomError> {
    Ok(CastironConfig {
        auto_dl_new: true,
        auto_rm_after_listen: true,
        dark_mode: true,
    })
}

pub fn create_config(config: CastironConfig) -> Result<(), CustomError> {
    let config_file_path = Path::new("./castiron_config.json");
    let config_file = File::create(config_file_path)?;
    let mut writer = BufWriter::new(config_file);
    to_writer(&mut writer, &config)?;
    writer.flush()?;
    Ok(())
}
