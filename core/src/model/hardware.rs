use serde::{Deserialize, Serialize};

use super::architecture::Architecture;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Hardware {
    pub architecture: Option<Architecture>,
    pub board_id: Option<String>,
    pub device_id: Option<String>,
    pub model_identifier: Option<String>,
}
