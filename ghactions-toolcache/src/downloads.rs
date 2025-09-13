//! # Downloading artifacts from GitHub
//!
//! The main functionality of this module is to download assets from GitHub releases.
//!
use octocrab::models::repos::Asset;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

use super::ToolCache;
use crate::ToolCacheError;

impl ToolCache {
    /// Get the tool cache client for downloads
    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Download an asset from a release
    pub async fn download_asset(
        &self,
        asset: &Asset,
        output: impl Into<PathBuf>,
    ) -> Result<(), ToolCacheError> {
        let output = output.into();
        log::debug!("Downloading asset to {:?}", output);

        let url = asset.browser_download_url.clone();
        let content_type = asset.content_type.clone();
        log::debug!("Downloading asset from {:?}", url);

        let mut file = tokio::fs::File::create(&output).await?;

        // TODO: GitHub auth for private repos

        let mut successful = false;
        let mut counter = self.retry_count;

        while counter > 0 {
            log::debug!("Attempting download, retries left: {}", counter);
            counter -= 1;

            let mut resp = self
                .client
                .get(url.clone())
                .header(
                    http::header::ACCEPT,
                    http::header::HeaderValue::from_str(&content_type)?,
                )
                .header(
                    http::header::USER_AGENT,
                    http::header::HeaderValue::from_str("ghactions")?,
                )
                .send()
                .await?;

            if resp.status().is_server_error() {
                log::warn!(
                    "Server error downloading asset: {:?}, retrying... {}",
                    resp.status(),
                    counter
                );
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }

            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
            }

            log::debug!("Download complete");
            successful = true;
            break;
        }

        if !successful {
            log::error!("Failed to download asset: {:?}", url);
            return Err(ToolCacheError::DownloadError(format!(
                "Failed to download asset: {:?}",
                url
            )));
        }

        Ok(())
    }
}
