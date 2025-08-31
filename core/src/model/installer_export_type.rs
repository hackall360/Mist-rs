use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallerExportType {
    Application,
    DiskImage,
    #[serde(rename = "ISO")]
    Iso,
    Package,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let t = InstallerExportType::Iso;
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, "\"ISO\"");
        let de: InstallerExportType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, t);
    }
}
