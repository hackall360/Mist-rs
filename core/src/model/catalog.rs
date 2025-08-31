use serde::{Deserialize, Serialize};

use super::catalog_type::CatalogType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "type")]
    pub catalog_type: CatalogType,
    pub standard: bool,
    pub customer_seed: bool,
    pub developer_seed: bool,
    pub public_seed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_catalog() {
        let data = r#"{
            "type": "macOS Ventura",
            "standard": true,
            "customer_seed": false,
            "developer_seed": false,
            "public_seed": false
        }"#;
        let catalog: Catalog = serde_json::from_str(data).unwrap();
        assert!(matches!(catalog.catalog_type, CatalogType::Ventura));
        assert!(catalog.standard);
        assert!(!catalog.customer_seed);
        assert!(!catalog.developer_seed);
        assert!(!catalog.public_seed);
        let serialized = serde_json::to_string(&catalog).unwrap();
        let round: Catalog = serde_json::from_str(&serialized).unwrap();
        assert_eq!(catalog, round);
    }
}
