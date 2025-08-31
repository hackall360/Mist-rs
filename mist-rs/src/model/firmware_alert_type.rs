use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FirmwareAlertType {
    #[serde(rename = "Compatiblity")]
    Compatibility,
    #[serde(rename = "Helper Tool")]
    HelperTool,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let t = FirmwareAlertType::HelperTool;
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, "\"Helper Tool\"");
        let de: FirmwareAlertType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, t);
    }
}
