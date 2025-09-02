use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressAlertType {
    #[serde(rename = "Cancel")]
    Cancel,
    #[serde(rename = "Error")]
    Error,
}
