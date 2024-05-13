use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CastironConfig {
    pub auto_dl_new: bool,
    pub auto_rm_after_listen: bool,
    pub dark_mode: bool,
}
