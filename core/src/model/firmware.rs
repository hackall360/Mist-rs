use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Firmware {
    pub version: String,
    pub build: String,
    pub shasum: String,
    pub size: u64,
    pub url: String,
    pub date: String,
    pub signed: bool,
    pub compatible: bool,
}
