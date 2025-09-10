//! Tool Cache

use glob::{MatchOptions, glob, glob_with};
use http::Uri;
use log::debug;
use octocrab::models::pulls::Head;
#[cfg(feature = "octocrab")]
use octocrab::models::repos::Asset;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use super::{Tool, ToolCacheArch, platform::ToolPlatform};
use crate::ActionsError;

/// Number of times to retry a download
const RETRY_COUNT: u8 = 10;

/// Linux and MacOS Tool Cache Paths
#[cfg(target_family = "unix")]
const TOOL_CACHE_PATHS: [&str; 3] = [
    "/opt/hostedtoolcache",
    "/usr/local/share/toolcache",
    "/tmp/toolcache",
];
/// Windows Tool Cache Paths
#[cfg(target_family = "windows")]
const TOOL_CACHE_PATHS: [&str; 3] = [
    "C:\\hostedtoolcache",
    "C:\\Program Files\\toolcache",
    "C:\\tmp\\toolcache",
];

/// Tool Cache
#[derive(Debug, Clone)]
pub struct ToolCache {
    /// Tool Cache Path
    pub tool_cache: PathBuf,

    /// Number of times to retry a download
    pub retry_count: u8,
}

impl ToolCache {
    /// Create a new Tool Cache
    ///
    /// This will either use the `RUNNER_TOOL_CACHE` environment variable or
    /// it will try to find the tool cache in the default locations.
    ///
    /// There are 3 default locations:
    ///  
    /// - `/opt/hostedtoolcache` (Unix)
    /// - `/usr/local/share/toolcache` (Unix)
    /// - `/tmp/toolcache` (Unix)
    /// - `C:\\hostedtoolcache` (Windows)
    /// - `C:\\Program Files\\toolcache` (Windows)
    /// - `C:\\tmp\\toolcache` (Windows)
    ///
    /// If no locations are found or writeable, it will create a new tool cache
    /// in the current directory at `./.toolcache`.
    ///
    pub fn new() -> Self {
        let tool_cache = std::env::var("RUNNER_TOOL_CACHE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                TOOL_CACHE_PATHS
                    .iter()
                    .find_map(|path| {
                        let path = PathBuf::from(path);
                        // Exists and can be written to
                        if let Err(err) = std::fs::create_dir_all(&path) {
                            log::trace!("Error creating tool cache dir: {:?}", err);
                            None
                        } else {
                            log::debug!("Using tool cache found at: {:?}", path);
                            Some(path)
                        }
                    })
                    .unwrap_or_else(|| PathBuf::from("./.toolcache").canonicalize().unwrap())
            });

        if !tool_cache.exists() {
            log::debug!("Creating tool cache at: {:?}", tool_cache);
            std::fs::create_dir_all(&tool_cache).unwrap_or_else(|_| {
                panic!("Failed to create tool cache directory: {:?}", tool_cache)
            });
        }

        Self {
            tool_cache,
            retry_count: RETRY_COUNT,
        }
    }

    /// Get the platform for the tool cache
    pub fn platform(&self) -> ToolPlatform {
        ToolPlatform::from_current_os()
    }

    /// Get the architecture for the tool cache
    pub fn arch(&self) -> ToolCacheArch {
        match std::env::consts::ARCH {
            "x86_64" | "amd64" => ToolCacheArch::X64,
            "aarch64" => ToolCacheArch::ARM64,
            _ => ToolCacheArch::Any,
        }
    }

    /// Get the Tool Cache Path
    pub fn get_tool_cache(&self) -> &PathBuf {
        &self.tool_cache
    }

    /// Find a tool in the cache
    pub async fn find(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Tool, ActionsError> {
        match self.platform() {
            ToolPlatform::Windows => self.find_with_arch(tool, version, ToolCacheArch::X64).await,
            ToolPlatform::Linux => self.find_with_arch(tool, version, ToolCacheArch::X64).await,
            ToolPlatform::MacOS => {
                self.find_with_arch(tool, version, ToolCacheArch::ARM64)
                    .await
            }
            ToolPlatform::Any => self.find_with_arch(tool, version, ToolCacheArch::Any).await,
        }
    }

    /// Find all versions of a tool in the cache
    pub async fn find_all_version(
        &self,
        tool: impl Into<String>,
    ) -> Result<Vec<Tool>, ActionsError> {
        Tool::find(self.get_tool_cache(), tool, "*", ToolCacheArch::Any)
    }

    /// Find a tool in the cache with a specific architecture
    pub async fn find_with_arch(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> Result<Tool, ActionsError> {
        let tool = tool.into();
        Tool::find(self.get_tool_cache(), tool.clone(), version, arch)?
            .into_iter()
            .find(|t| t.name() == tool)
            .ok_or_else(|| crate::errors::ActionsError::ToolNotFound(tool))
    }

    /// Create a path for the tool in the cache to be used
    pub fn new_tool_path(&self, tool: impl Into<String>, version: impl Into<String>) -> PathBuf {
        Tool::tool_path(self.get_tool_cache(), tool, version, self.arch())
    }

    /// Set the number of times to retry a download (default is 10)
    pub fn set_retry_count(&mut self, count: u8) {
        self.retry_count = count;
    }

    /// Download an asset from a release
    #[cfg(feature = "octocrab")]
    pub async fn download_asset(
        &self,
        asset: &Asset,
        output: impl Into<PathBuf>,
    ) -> Result<(), ActionsError> {
        use tokio::io::AsyncWriteExt;

        let output = output.into();
        log::debug!("Downloading asset to {:?}", output);

        let url = asset.browser_download_url.clone();
        let content_type = asset.content_type.clone();
        log::debug!("Downloading asset from {:?}", url);

        let mut file = tokio::fs::File::create(&output).await?;

        // TODO: GitHub auth for private repos

        let mut successful = false;
        let mut counter = self.retry_count;
        let client = reqwest::Client::new();

        while counter > 0 {
            debug!("Attempting download, retries left: {}", counter);
            counter -= 1;

            let mut resp = client
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
            return Err(ActionsError::DownloadError(format!(
                "Failed to download asset: {:?}",
                url
            )));
        }

        Ok(())
    }
}

impl From<&str> for ToolCache {
    fn from(cache: &str) -> Self {
        let tool_cache = PathBuf::from(cache);
        if !tool_cache.exists() {
            panic!("Tool Cache does not exist: {:?}", tool_cache);
        }
        Self {
            tool_cache,
            retry_count: RETRY_COUNT,
        }
    }
}

impl From<PathBuf> for ToolCache {
    fn from(value: PathBuf) -> Self {
        let tool_cache = value;
        if !tool_cache.exists() {
            panic!("Tool Cache does not exist: {:?}", tool_cache);
        }
        Self {
            tool_cache,
            retry_count: RETRY_COUNT,
        }
    }
}

impl Default for ToolCache {
    fn default() -> Self {
        let mut tool_cache = std::env::var("RUNNER_TOOL_CACHE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/opt/hostedtoolcache"));

        if !tool_cache.is_absolute() {
            tool_cache = tool_cache.canonicalize().unwrap();
        }

        Self {
            tool_cache,
            retry_count: RETRY_COUNT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_toolcache() -> (PathBuf, ToolCache) {
        // working dir + examples/toolcache
        let cwd = std::env::current_dir()
            .unwrap()
            .join("..")
            .canonicalize()
            .unwrap();

        (cwd.clone(), ToolCache::from(cwd.join("examples/toolcache")))
    }

    #[test]
    fn test_tool_cache() {
        // Default
        let tool_cache = ToolCache::default();
        assert_eq!(
            tool_cache.get_tool_cache().to_str().unwrap(),
            "/opt/hostedtoolcache"
        );
    }

    #[tokio::test]
    async fn test_find_all_version() {
        let (_cwd, tool_cache) = local_toolcache();
        let versions = tool_cache.find_all_version("node").await.unwrap();
        assert!(!versions.is_empty());
    }
}
