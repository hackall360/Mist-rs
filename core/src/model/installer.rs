use serde::{Deserialize, Serialize};

use super::package::Package;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Installer {
    #[serde(rename = "Identifier")]
    pub id: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Build")]
    pub build: String,
    #[serde(rename = "PostDate")]
    pub date: String,
    #[serde(rename = "DistributionURL")]
    pub distribution_url: String,
    #[serde(rename = "DistributionSize")]
    pub distribution_size: u64,
    #[serde(rename = "Packages")]
    pub packages: Vec<Package>,
    #[serde(rename = "BoardIDs")]
    pub board_ids: Vec<String>,
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    #[serde(rename = "UnsupportedModelIdentifiers")]
    pub unsupported_model_identifiers: Vec<String>,
}
