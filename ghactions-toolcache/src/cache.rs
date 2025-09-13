//! Tool Cache

use std::path::PathBuf;

use super::{Tool, ToolCacheArch, platform::ToolPlatform};
use crate::ToolCacheError;
use crate::builder::ToolCacheBuilder;

/// Number of times to retry a download
pub(crate) const RETRY_COUNT: u8 = 10;

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
    pub(crate) tool_cache: PathBuf,

    /// Platform Architecture
    pub(crate) arch: ToolCacheArch,

    /// Platform (OS)
    pub(crate) platform: ToolPlatform,

    /// Number of times to retry a download
    pub(crate) retry_count: u8,

    /// Client to use for downloads
    #[cfg(feature = "download")]
    pub(crate) client: reqwest::Client,
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
        Self::default()
    }

    /// Create a new ToolCacheBuilder
    pub fn build() -> ToolCacheBuilder {
        ToolCacheBuilder::new()
    }

    /// Get the platform for the tool cache
    ///
    /// By default this is set to the current platform of the system.
    /// You can override this by using the `platform` method on the `ToolCacheBuilder`.
    pub fn platform(&self) -> ToolPlatform {
        self.platform
    }

    /// Get the architecture for the tool cache
    ///
    /// By default this is set to the current architecture of the system.
    /// You can override this by using the `arch` method on the `ToolCacheBuilder`.
    pub fn arch(&self) -> ToolCacheArch {
        self.arch
    }

    /// Get the Tool Cache Path
    ///
    /// This is either set by the `RUNNER_TOOL_CACHE` environment variable
    /// or it is one of the default locations.
    pub fn get_tool_cache(&self) -> &PathBuf {
        &self.tool_cache
    }

    /// Find a tool in the cache
    pub async fn find(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Tool, ToolCacheError> {
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
    ) -> Result<Vec<Tool>, ToolCacheError> {
        Tool::find(self.get_tool_cache(), tool, "*", ToolCacheArch::Any)
    }

    /// Find a tool in the cache with a specific architecture
    pub async fn find_with_arch(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> Result<Tool, ToolCacheError> {
        let tool = tool.into();
        let version = version.into();
        let arch = arch.into();

        Tool::find(self.get_tool_cache(), tool.clone(), &version, &arch)?
            .into_iter()
            .find(|t| t.name() == tool)
            .ok_or_else(|| crate::ToolCacheError::ToolNotFound {
                name: tool,
                version,
                arch: Some(arch),
            })
    }

    /// Create a path for the tool in the cache to be used
    pub fn new_tool_path(&self, tool: impl Into<String>, version: impl Into<String>) -> PathBuf {
        Tool::tool_path(self.get_tool_cache(), tool, version, self.arch())
    }

    /// Set the number of times to retry a download (default is 10)
    #[deprecated(since = "0.17.0", note = "Use the ToolCacheBuilder instead")]
    pub fn set_retry_count(&mut self, count: u8) {
        self.retry_count = count;
    }
}

/// Get the tool cache path
pub(crate) fn get_tool_cache_path() -> PathBuf {
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
        std::fs::create_dir_all(&tool_cache)
            .unwrap_or_else(|_| panic!("Failed to create tool cache directory: {:?}", tool_cache));
    }
    tool_cache
}

impl From<&str> for ToolCache {
    fn from(cache: &str) -> Self {
        let tool_cache = PathBuf::from(cache);
        if !tool_cache.exists() {
            panic!("Tool Cache does not exist: {:?}", tool_cache);
        }
        Self {
            tool_cache,
            ..Default::default()
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
            ..Default::default()
        }
    }
}

impl Default for ToolCache {
    fn default() -> Self {
        let tool_cache = get_tool_cache_path();

        Self {
            tool_cache,
            retry_count: RETRY_COUNT,
            arch: match std::env::consts::ARCH {
                "x86_64" | "amd64" => ToolCacheArch::X64,
                "aarch64" => ToolCacheArch::ARM64,
                _ => ToolCacheArch::Any,
            },
            platform: ToolPlatform::from_current_os(),
            #[cfg(feature = "download")]
            client: reqwest::Client::new(),
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
        if let Ok(env_path) = std::env::var("RUNNER_TOOL_CACHE") {
            assert_eq!(tool_cache.get_tool_cache(), &PathBuf::from(env_path));
        } else {
            assert!(tool_cache.get_tool_cache().exists());
        }
    }

    #[tokio::test]
    async fn test_find_all_version() {
        let (_cwd, tool_cache) = local_toolcache();
        let versions = tool_cache.find_all_version("node").await.unwrap();
        assert!(!versions.is_empty());
    }
}
