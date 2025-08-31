use mist_rs::helpers::download_manager::DownloadManager;
use tempfile::tempdir;

#[tokio::test]
async fn downloads_file() {
    let dir = tempdir().expect("temp dir");
    let dest = dir.path().join("rust-logo.png");
    let manager = DownloadManager::new();
    manager
        .download(
            "https://www.rust-lang.org/logos/rust-logo-32x32.png",
            &dest,
        )
        .await
        .expect("download");
    assert!(dest.exists());
    assert!(dest.metadata().unwrap().len() > 0);
}
