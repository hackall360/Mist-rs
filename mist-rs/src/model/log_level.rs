use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let level = LogLevel::Warning;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"warning\"");
        let de: LogLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(de, level);
    }
}
