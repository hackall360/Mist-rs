use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallerAlertType {
    #[serde(rename = "Compatiblity")]
    Compatibility,
    #[serde(rename = "Helper Tool")]
    HelperTool,
    #[serde(rename = "Full Disk Access")]
    FullDiskAccess,
    #[serde(rename = "Cache Directory")]
    CacheDirectory,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let t = InstallerAlertType::Error;
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, "\"Error\"");
        let de: InstallerAlertType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, t);
    }
}
