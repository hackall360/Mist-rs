use reqwest::Client;
use std::path::Path;
use tokio::fs;

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
        let bytes = self.client.get(url).send().await?.bytes().await?;
        fs::write(dest, &bytes).await?;
        Ok(())
    }
}
