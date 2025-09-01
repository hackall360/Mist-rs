use mist_core::helpers::download_manager::DownloadManager;
use tempfile::tempdir;
use mockito::Server;

#[tokio::test]
async fn downloads_file() {
    let dir = tempdir().expect("temp dir");
    let dest = dir.path().join("file.txt");

    let mut server = Server::new_async().await;
    let _m = server
        .mock("GET", "/file")
        .with_status(200)
        .with_body("test-data")
        .create_async()
        .await;

    let manager = DownloadManager::new();
    let url = format!("{}/file", server.url());
    manager.download(&url, &dest).await.expect("download");

    assert!(dest.exists());
    let contents = tokio::fs::read_to_string(&dest).await.unwrap();
    assert_eq!(contents, "test-data");
}
