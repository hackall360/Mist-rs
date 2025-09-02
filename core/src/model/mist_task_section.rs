use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MistTaskSection {
    #[serde(rename = "Download")]
    Download,
    #[serde(rename = "Setup")]
    Setup,
    #[serde(rename = "Application")]
    Application,
    #[serde(rename = "Disk Image")]
    DiskImage,
    #[serde(rename = "ISO")]
    Iso,
    #[serde(rename = "Package")]
    Package,
    #[serde(rename = "Bootable Installer")]
    BootableInstaller,
    #[serde(rename = "Cleanup")]
    Cleanup,
}
