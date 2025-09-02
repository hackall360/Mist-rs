use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingsInstallerCacheAlertType {
    #[serde(rename = "Confirmation")]
    Confirmation,
    #[serde(rename = "Error")]
    Error,
}
