use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallerSheetType {
    Download,
    #[serde(rename = "Volume Selection")]
    VolumeSelection,
    #[serde(rename = "Create Bootable Installer")]
    CreateBootableInstaller,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let t = InstallerSheetType::VolumeSelection;
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, "\"Volume Selection\"");
        let de: InstallerSheetType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, t);
    }
}
