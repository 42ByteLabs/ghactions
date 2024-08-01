//! # GitHub Actions Tool Cache
//!
//! ### Example
//!
//! ```no_run
//! # use anyhow::Result;
//! use ghactions::ToolCache;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! # std::env::set_var("RUNNER_TOOL_CACHE", "/opt/hostedtoolcache");
//!
//! let tool_cache = ToolCache::new();
//! let path = tool_cache.find("node", "14.17.0").await
//!     .expect("Failed to find tool in cache");
//!
//! println!("{:?}", path);
//! # Ok(())
//! # }
//! ```

use glob::{glob, glob_with, MatchOptions};
use log::debug;
use std::path::PathBuf;

pub mod arch;

pub use arch::ToolCacheArch;

/// Tool Cache
#[derive(Debug, Clone)]
pub struct ToolCache {
    /// Tool Cache Path
    pub tool_cache: PathBuf,
}

impl ToolCache {
    /// Create a new Tool Cache
    pub fn new() -> Self {
        Self::default()
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
    ) -> Result<PathBuf, crate::errors::ActionsError> {
        match std::env::consts::OS {
            "windows" => self.find_with_arch(tool, version, ToolCacheArch::X64).await,
            "linux" => self.find_with_arch(tool, version, ToolCacheArch::X64).await,
            "macos" => {
                self.find_with_arch(tool, version, ToolCacheArch::ARM64)
                    .await
            }
            _ => self.find_with_arch(tool, version, ToolCacheArch::Any).await,
        }
    }

    /// Find all versions of a tool in the cache
    pub async fn find_all_version(
        &self,
        tool: impl Into<String>,
    ) -> Result<Vec<PathBuf>, crate::errors::ActionsError> {
        self.find_matches(tool, "*", ToolCacheArch::Any)
            .into_iter()
            .map(|path| {
                debug!("Found Tool: {:?}", path);
                Ok(path)
            })
            .collect()
    }

    /// Find a tool in the cache with a specific architecture
    pub async fn find_with_arch(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> Result<PathBuf, crate::errors::ActionsError> {
        let tool = tool.into();
        self.find_matches(tool.clone(), version, arch)
            .into_iter()
            .next()
            .ok_or_else(|| crate::errors::ActionsError::ToolNotFound(tool))
    }

    /// Find all matches of a tool in the cache
    fn find_matches(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> Vec<PathBuf> {
        let tool = tool.into();
        let version = version.into();
        let arch = arch.into();

        let tool_path = self.tool_path(tool, version, arch);
        let tool_path_str = tool_path.to_str().unwrap();

        let mut results = vec![];

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };

        for entry in glob_with(tool_path_str, options).expect("Failed to read tool cache") {
            let path = entry.expect("Failed to read tool cache");
            if path.is_dir() && path.exists() {
                results.push(path);
            }
        }

        results
    }

    /// Get the path to a tool in the cache
    fn tool_path(
        &self,
        tool: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> PathBuf {
        // TODO: Validate the tool name
        let tool = tool.into();
        let mut version = version.into();
        // Replace x with *, this allows for dynamic versions
        if version.contains('x') {
            version = version.replace("x", "*");
        }
        let arch = match arch.into() {
            ToolCacheArch::X64 => "x64",
            ToolCacheArch::ARM64 => "arm64",
            ToolCacheArch::Any => "**",
        };
        // Trailling slash is important
        self.tool_cache.join(tool).join(version).join(arch).join("")
    }
}

impl From<&str> for ToolCache {
    fn from(cache: &str) -> Self {
        let tool_cache = PathBuf::from(cache);
        if !tool_cache.exists() {
            panic!("Tool Cache does not exist: {:?}", tool_cache);
        }
        Self { tool_cache }
    }
}

impl From<PathBuf> for ToolCache {
    fn from(value: PathBuf) -> Self {
        let tool_cache = value;
        if !tool_cache.exists() {
            panic!("Tool Cache does not exist: {:?}", tool_cache);
        }
        Self { tool_cache }
    }
}

impl Default for ToolCache {
    fn default() -> Self {
        let tool_cache = std::env::var("RUNNER_TOOL_CACHE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/opt/hostedtoolcache"));

        Self { tool_cache }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_toolcache() -> (PathBuf, ToolCache) {
        // working dir + examples/toolcache
        let cwd = PathBuf::from(std::env::current_dir().unwrap())
            .join("..")
            .canonicalize()
            .unwrap();

        (cwd.clone(), ToolCache::from(cwd.join("examples/toolcache")))
    }

    #[test]
    fn test_tool_cache() {
        // Default
        let tool_cache = ToolCache::new();
        assert_eq!(
            tool_cache.get_tool_cache().to_str().unwrap(),
            "/opt/hostedtoolcache"
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_tool_path() {
        let (cwd, tool_cache) = local_toolcache();

        // Static
        let path = tool_cache.tool_path("node", "12.7.0", "x64");
        assert_eq!(
            path.to_str().unwrap(),
            cwd.join("examples/toolcache/node/12.7.0/x64/")
                .to_str()
                .unwrap()
        );

        // Dynamic (version)
        let path = tool_cache.tool_path("node", "12.x", "x64");
        assert_eq!(
            path.to_str().unwrap(),
            cwd.join("examples/toolcache/node/12.*/x64/")
                .to_str()
                .unwrap()
        );

        // Dynamic (arch)
        let path = tool_cache.tool_path("node", "12.7.0", ToolCacheArch::Any);
        assert_eq!(
            path.to_str().unwrap(),
            cwd.join("examples/toolcache/node/12.7.0/**/")
                .to_str()
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_find_all_version() {
        let (_cwd, tool_cache) = local_toolcache();
        let versions = tool_cache.find_all_version("node").await.unwrap();
        assert!(!versions.is_empty());
    }
}
