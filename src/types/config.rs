use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CastironConfig {
    pub auto_dl_new: bool,
    pub auto_rm_after_listen: bool,
    pub theme: String,
}
