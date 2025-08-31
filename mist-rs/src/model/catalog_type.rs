use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CatalogType {
    #[serde(rename = "macOS Sequoia")]
    Sequoia,
    #[serde(rename = "macOS Sonoma")]
    Sonoma,
    #[serde(rename = "macOS Ventura")]
    Ventura,
    #[serde(rename = "macOS Monterey")]
    Monterey,
    #[serde(rename = "macOS Big Sur")]
    BigSur,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let typ = CatalogType::Ventura;
        let json = serde_json::to_string(&typ).unwrap();
        assert_eq!(json, "\"macOS Ventura\"");
        let de: CatalogType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, typ);
    }
}
