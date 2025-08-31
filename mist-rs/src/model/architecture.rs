use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    #[serde(rename = "arm64")]
    AppleSilicon,
    #[serde(rename = "x86_64")]
    Intel,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let arch = Architecture::Intel;
        let json = serde_json::to_string(&arch).unwrap();
        assert_eq!(json, "\"x86_64\"");
        let de: Architecture = serde_json::from_str(&json).unwrap();
        assert_eq!(de, arch);
    }
}
