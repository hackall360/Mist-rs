use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallerVolume {
    pub id: String,
    pub name: String,
    pub path: String,
    pub capacity: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_round_trip() {
        let vol = InstallerVolume {
            id: "vol1".into(),
            name: "Volume".into(),
            path: "/Volumes/Volume".into(),
            capacity: 1024,
        };
        let json = serde_json::to_string(&vol).unwrap();
        let de: InstallerVolume = serde_json::from_str(&json).unwrap();
        assert_eq!(vol, de);
    }
}
