use futures_util::TryStreamExt;
use reqwest::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct DownloadManager {
    client: Client,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn download(&self, url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if dest.exists() {
            return Ok(());
        }

        let response = self.client.get(url).send().await?;
        let mut file = File::create(dest).await?;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.try_next().await? {
            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        Ok(())
    }
}
