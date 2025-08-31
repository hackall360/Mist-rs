use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CatalogSeedType {
    Standard,
    Customer,
    Developer,
    Public,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let seed = CatalogSeedType::Developer;
        let json = serde_json::to_string(&seed).unwrap();
        assert_eq!(json, "\"Developer\"");
        let de: CatalogSeedType = serde_json::from_str(&json).unwrap();
        assert_eq!(de, seed);
    }
}
