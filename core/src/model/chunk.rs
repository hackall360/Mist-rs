use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chunk {
    pub size: u32,
    pub hash: Vec<u8>,
}

impl Chunk {
    pub fn shasum(&self) -> String {
        self.hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}
