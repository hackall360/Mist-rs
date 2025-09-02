use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefreshState {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "In Progress")]
    InProgress,
    #[serde(rename = "Complete")]
    Complete,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Error")]
    Error,
}
