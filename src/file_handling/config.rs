use crate::types::config::CastironConfig;
use crate::types::errors::CustomError;
use serde_json::{Deserializer, Serializer};

pub fn read_config() -> Result<CastironConfig, CustomError> {
    Ok(CastironConfig {
        auto_dl_new: true,
        auto_rm_after_listen: true,
        dark_mode: true,
    })
}

pub fn create_config(config: CastironConfig) -> Result<(), CustomError> {
    Ok(())
}
