use mist_core::helpers::{chunklist::Chunklist, validator};
use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn verifies_chunklist() {
    let dir = tempdir().unwrap();
    let pkg_path = dir.path().join("pkg.bin");
    fs::write(&pkg_path, b"hello world").await.unwrap();

    // compute sha256
    let digest = ring::digest::digest(&ring::digest::SHA256, b"hello world");
    let mut hash = [0u8; 32];
    hash.copy_from_slice(digest.as_ref());

    // build chunklist bytes for single chunk
    let mut data = Vec::new();
    data.extend_from_slice(&0x4C4B4E43u32.to_be_bytes());
    data.extend_from_slice(&0x00000024u32.to_be_bytes());
    data.extend_from_slice(&[0x01, 0x01, 0x02, 0x00]);
    let total_chunks: u64 = 1;
    data.extend_from_slice(&total_chunks.to_be_bytes());
    data.extend_from_slice(&0x0000000000000024u64.to_be_bytes());
    let signature_offset = 0x24 + 0x24; // header + one chunk
    data.extend_from_slice(&(signature_offset as u64).to_be_bytes());
    data.extend_from_slice(&(11u32).to_be_bytes());
    data.extend_from_slice(&hash);

    let chunklist = Chunklist::from_bytes(&data).unwrap();
    validator::validate_package(&pkg_path, &chunklist)
        .await
        .expect("validation");
}
