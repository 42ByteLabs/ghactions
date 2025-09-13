//! # ToolCacheBuilder
//!
//! A builder for the ToolCache struct.
//!
//! This allows you to customize the ToolCache instance before creating it.
//!
//! # Example
//!
//! ```no_run
//! # use anyhow::Result;
//! use ghactions_toolcache::ToolCache;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//!
//! // Create a new ToolCache instance using the builder
//! let tool_cache = ToolCache::build()
//!     .retry_count(5)
//!     .client(reqwest::Client::new())
//!     .build();
//!
//! # Ok(())
//! # }
//! ```
use std::path::PathBuf;

use crate::{
    ToolCache, ToolCacheArch, ToolPlatform,
    cache::{RETRY_COUNT, get_tool_cache_path},
};

#[derive(Debug, Clone, Default)]
pub struct ToolCacheBuilder {
    pub(crate) tool_cache: Option<PathBuf>,
    pub(crate) arch: Option<crate::ToolCacheArch>,
    pub(crate) platform: Option<crate::platform::ToolPlatform>,

    pub(crate) retry_count: Option<u8>,
    pub(crate) client: Option<reqwest::Client>,
}

impl ToolCacheBuilder {
    /// Create a new ToolCacheBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the path to the tool cache directory.
    ///
    /// # Parameters
    /// - `path`: The path to use for the tool cache directory.
    pub fn tool_cache(mut self, path: impl Into<PathBuf>) -> Self {
        self.tool_cache = Some(path.into());
        self
    }

    /// Sets the architecture for the tool cache.
    ///
    /// # Parameters
    /// - `arch`: The architecture to use (e.g., x64, arm64).
    pub fn arch(mut self, arch: crate::ToolCacheArch) -> Self {
        self.arch = Some(arch);
        self
    }

    /// Sets the platform for the tool cache.
    ///
    /// # Parameters
    /// - `platform`: The platform to use (e.g., Windows, Linux, macOS).
    pub fn platform(mut self, platform: crate::platform::ToolPlatform) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Sets the number of retry attempts for cache operations.
    ///
    /// # Parameters
    /// - `count`: The number of retries to attempt.
    pub fn retry_count(mut self, count: u8) -> Self {
        self.retry_count = Some(count);
        self
    }

    /// Sets the HTTP client to use for downloading tools.
    ///
    /// # Parameters
    /// - `client`: The `reqwest::Client` instance to use.
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Build the ToolCache
    pub fn build(&self) -> ToolCache {
        let tool_cache = self
            .tool_cache
            .clone()
            .unwrap_or_else(|| get_tool_cache_path());
        let arch = self
            .arch
            .clone()
            .unwrap_or_else(|| match std::env::consts::ARCH {
                "x86_64" | "amd64" => ToolCacheArch::X64,
                "aarch64" => ToolCacheArch::ARM64,
                _ => ToolCacheArch::Any,
            });

        let platform = self
            .platform
            .unwrap_or_else(|| ToolPlatform::from_current_os());

        ToolCache {
            tool_cache,
            arch,
            platform,
            retry_count: self.retry_count.unwrap_or(RETRY_COUNT),
            #[cfg(feature = "download")]
            client: self.client.clone().unwrap_or_else(reqwest::Client::new),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
