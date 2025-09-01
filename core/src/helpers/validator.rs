use std::path::Path;

use ring::digest::{self, SHA256};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::helpers::chunklist::Chunklist;

pub async fn validate_package(
    package: &Path,
    chunklist: &Chunklist,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(package).await?;
    for chunk in &chunklist.chunks {
        let mut buf = vec![0u8; chunk.size as usize];
        file.read_exact(&mut buf).await?;
        let digest = digest::digest(&SHA256, &buf);
        if digest.as_ref() != chunk.hash {
            return Err("invalid chunk checksum".into());
        }
    }
    Ok(())
}

pub async fn validate_from_url(
    package: &Path,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    let chunklist = Chunklist::from_bytes(&bytes)?;
    validate_package(package, &chunklist).await
}
