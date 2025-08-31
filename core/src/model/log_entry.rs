use serde::{Deserialize, Serialize};

use super::log_level::LogLevel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_log_entry() {
        let data = r#"{
            "id": "123",
            "timestamp": "2024-02-11T00:00:00Z",
            "level": "info",
            "message": "Message"
        }"#;
        let entry: LogEntry = serde_json::from_str(data).unwrap();
        assert_eq!(entry.id, "123");
        assert!(matches!(entry.level, LogLevel::Info));
        assert_eq!(entry.message, "Message");
        let serialized = serde_json::to_string(&entry).unwrap();
        let round: LogEntry = serde_json::from_str(&serialized).unwrap();
        assert_eq!(round, entry);
    }
}
