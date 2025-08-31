use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DownloadType {
    Firmware,
    Installer,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let t = DownloadType::Firmware;
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, "\"Firmware\"");
        let de: DownloadType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, t);
    }
}
