use serde::{Deserialize, Serialize};

use super::chunk::Chunk;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunklist {
    pub magic_header: u32,
    pub header_size: u32,
    pub file_version: u8,
    pub chunk_method: u8,
    pub signature_method: u8,
    pub padding: u8,
    pub total_chunks: u64,
    pub chunks_offset: u64,
    pub signature_offset: u64,
    pub chunks: Vec<Chunk>,
    pub signature: Vec<u8>,
}
