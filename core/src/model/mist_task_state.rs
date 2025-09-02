use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistTaskState {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "In Progress")]
    InProgress,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Error")]
    Error,
}
