use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "IntegrityDataURL")]
    pub integrity_data_url: Option<String>,
    #[serde(rename = "IntegrityDataSize")]
    pub integrity_data_size: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_package() {
        let data = r#"{
            "URL": "https://example.com/file.pkg",
            "Size": 123,
            "IntegrityDataURL": null,
            "IntegrityDataSize": null
        }"#;
        let pkg: Package = serde_json::from_str(data).unwrap();
        assert_eq!(pkg.url, "https://example.com/file.pkg");
        assert_eq!(pkg.size, 123);
        assert!(pkg.integrity_data_url.is_none());
        let serialized = serde_json::to_string(&pkg).unwrap();
        let round: Package = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pkg, round);
    }
}
