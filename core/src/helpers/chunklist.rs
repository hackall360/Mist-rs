use std::convert::TryInto;

pub struct Chunk {
    pub size: u32,
    pub hash: [u8; 32],
}

pub struct Chunklist {
    pub chunks: Vec<Chunk>,
}

impl Chunklist {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if data.len() < 0x24 {
            return Err("chunklist too small".into());
        }

        let magic = u32::from_be_bytes(data[0x00..0x04].try_into()?);
        if magic != 0x4C4B4E43 {
            return Err("invalid magic header".into());
        }

        let total_chunks = u64::from_be_bytes(data[0x0C..0x14].try_into()?);
        let chunks_offset = u64::from_be_bytes(data[0x14..0x1C].try_into()?);
        let signature_offset = u64::from_be_bytes(data[0x1C..0x24].try_into()?);

        if chunks_offset as usize != 0x24 {
            return Err("invalid chunks offset".into());
        }

        if signature_offset as usize != chunks_offset as usize + total_chunks as usize * 0x24 {
            return Err("invalid signature offset".into());
        }

        let mut chunks = Vec::new();
        let mut pos = chunks_offset as usize;
        for _ in 0..total_chunks {
            let size = u32::from_be_bytes(data[pos..pos + 4].try_into()?);
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&data[pos + 4..pos + 36]);
            chunks.push(Chunk { size, hash });
            pos += 36;
        }

        Ok(Self { chunks })
    }
}
