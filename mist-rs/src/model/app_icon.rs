use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AppIcon {
    Monterey,
    Ventura,
    Sonoma,
    Sequoia,
}

impl Default for AppIcon {
    fn default() -> Self {
        AppIcon::Sequoia
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let icon = AppIcon::Ventura;
        let json = serde_json::to_string(&icon).unwrap();
        assert_eq!(json, "\"Ventura\"");
        let de: AppIcon = serde_json::from_str(&json).unwrap();
        assert_eq!(de, icon);
    }
}
