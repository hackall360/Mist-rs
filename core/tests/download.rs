use mist_core::helpers::download_manager::DownloadManager;
use mockito::Server;
use tempfile::tempdir;

#[tokio::test]
async fn downloads_file() {
    let dir = tempdir().expect("temp dir");
    let dest = dir.path().join("file.txt");

    let mut server = Server::new_async().await;
    let _head = server
        .mock("HEAD", "/file")
        .with_status(200)
        .with_header("content-length", "9")
        .create_async()
        .await;
    let _get = server
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

#[tokio::test]
async fn retries_on_failure() {
    let dir = tempdir().expect("temp dir");
    let dest = dir.path().join("file.txt");

    let mut server = Server::new_async().await;
    let _head1 = server
        .mock("HEAD", "/retry")
        .with_status(200)
        .with_header("content-length", "9")
        .create_async()
        .await;
    let _get_fail = server
        .mock("GET", "/retry")
        .with_status(500)
        .create_async()
        .await;
    let _head2 = server
        .mock("HEAD", "/retry")
        .with_status(200)
        .with_header("content-length", "9")
        .create_async()
        .await;
    let _get_ok = server
        .mock("GET", "/retry")
        .with_status(200)
        .with_body("test-data")
        .create_async()
        .await;

    let manager = DownloadManager::new();
    let url = format!("{}/retry", server.url());
    manager
        .download_with_retry(&url, &dest, 2, 0)
        .await
        .expect("download");

    assert!(dest.exists());
    let contents = tokio::fs::read_to_string(&dest).await.unwrap();
    assert_eq!(contents, "test-data");
}

#[tokio::test]
async fn reports_progress() {
    let dir = tempdir().expect("temp dir");
    let dest = dir.path().join("file.txt");

    let mut server = Server::new_async().await;
    let _head = server
        .mock("HEAD", "/progress")
        .with_status(200)
        .with_header("content-length", "9")
        .create_async()
        .await;
    let _get = server
        .mock("GET", "/progress")
        .with_status(200)
        .with_body("test-data")
        .create_async()
        .await;

    let manager = DownloadManager::new();
    let progress_before = manager.progress().await;
    assert_eq!(progress_before, 0.0);

    let url = format!("{}/progress", server.url());
    manager
        .download_with_retry(&url, &dest, 1, 0)
        .await
        .expect("download");

    let progress_after = manager.progress().await;
    assert_eq!(progress_after, 1.0);
}
