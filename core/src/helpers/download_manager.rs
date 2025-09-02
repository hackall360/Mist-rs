use futures_util::TryStreamExt;
use reqwest::header::{CONTENT_LENGTH, RANGE};
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Default)]
struct DownloadState {
    retries: usize,
    delay: Duration,
    downloaded: u64,
    total: u64,
}

pub struct DownloadManager {
    client: Client,
    state: Arc<Mutex<DownloadState>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            state: Arc::new(Mutex::new(DownloadState::default())),
        }
    }

    pub async fn download(&self, url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.download_with_retry(url, dest, 1, 0).await
    }

    pub async fn download_with_retry(
        &self,
        url: &str,
        dest: &Path,
        retries_max: usize,
        delay_secs: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut state = self.state.lock().await;
            state.retries = 0;
            state.delay = Duration::from_secs(delay_secs);
            state.downloaded = 0;
            state.total = 0;
        }

        let head = self.client.head(url).send().await?.error_for_status()?;
        let total = head
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        {
            let mut state = self.state.lock().await;
            state.total = total;
        }

        if dest.exists() {
            let meta = tokio::fs::metadata(dest).await?;
            if meta.len() == total {
                let mut state = self.state.lock().await;
                state.downloaded = total;
                return Ok(());
            }
        }

        let delay = Duration::from_secs(delay_secs);
        let mut retries = 0usize;
        loop {
            let downloaded = if dest.exists() {
                tokio::fs::metadata(dest).await?.len()
            } else {
                0
            };

            {
                let mut state = self.state.lock().await;
                state.downloaded = downloaded;
            }

            let mut file = if downloaded > 0 {
                OpenOptions::new().append(true).open(dest).await?
            } else {
                File::create(dest).await?
            };

            let mut req = self.client.get(url);
            if downloaded > 0 {
                req = req.header(RANGE, format!("bytes={}-", downloaded));
            }

            let resp = req.send().await;

            match resp {
                Ok(resp) => match resp.error_for_status() {
                    Ok(ok) => {
                        let mut stream = ok.bytes_stream();
                        let mut downloaded = downloaded;
                        while let Some(chunk) = stream.try_next().await? {
                            file.write_all(&chunk).await?;
                            downloaded += chunk.len() as u64;
                            let mut state = self.state.lock().await;
                            state.downloaded = downloaded;
                        }
                        file.flush().await?;
                        return Ok(());
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }

            retries += 1;
            {
                let mut state = self.state.lock().await;
                state.retries = retries;
            }
            if retries >= retries_max {
                return Err("maximum retries reached".into());
            }
            sleep(delay).await;
        }
    }

    pub async fn progress(&self) -> f64 {
        let state = self.state.lock().await;
        if state.total == 0 {
            0.0
        } else {
            state.downloaded as f64 / state.total as f64
        }
    }
}
