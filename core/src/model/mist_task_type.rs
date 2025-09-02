use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistTaskType {
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "Verify")]
    Verify,
    #[serde(rename = "Configure")]
    Configure,
    #[serde(rename = "Move")]
    Move,
    #[serde(rename = "Copy")]
    Copy,
    #[serde(rename = "Create")]
    Create,
    #[serde(rename = "Remove")]
    Remove,
    #[serde(rename = "Codesign")]
    Codesign,
    #[serde(rename = "Mount")]
    Mount,
    #[serde(rename = "Unmount")]
    Unmount,
    #[serde(rename = "Convert")]
    Convert,
}
