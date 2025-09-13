//! # GitHub Actions Tool Cache
//!
//! A Rust library for managing a tool cache similar to the one used in
//! GitHub Actions.
//!
//! This library allows you to find, add, download, and manage tools in a
//! local cache directory. It supports multiple platforms and architectures.
//!
//!
//! ### Example
//!
//! ```no_run
//! # use anyhow::Result;
//! use ghactions::ToolCache;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//!
//! // Create a new ToolCache instance
//! let tool_cache = ToolCache::new();
//!
//! // Find a tool in the cache
//! let path = tool_cache.find("node", "latest").await
//!     .expect("Failed to find tool in cache");
//!
//! // Find a specific version of a tool in the cache
//! let path = tool_cache.find("node", "20.0.0").await
//!     .expect("Failed to find tool in cache");
//!
//! # Ok(())
//! # }
//! ```
#![deny(missing_docs, unsafe_code)]

pub mod arch;
pub mod archives;
pub mod builder;
pub mod cache;
#[cfg(feature = "download")]
pub mod downloads;
pub mod platform;
pub mod tool;

pub use arch::ToolCacheArch;
pub use cache::ToolCache;
pub use platform::ToolPlatform;
pub use tool::Tool;

/// Tool cache errors
#[derive(Debug, thiserror::Error)]
pub enum ToolCacheError {
    /// Tool not found in cache
    #[error("Tool not found in cache: {name} {version} {arch:?}")]
    ToolNotFound {
        /// Tool name
        name: String,
        /// Tool version
        version: String,
        /// Tool architecture (if specified)
        arch: Option<ToolCacheArch>,
    },

    /// I/O Error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// HTTP Error
    #[cfg(feature = "api")]
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Header Error
    #[cfg(feature = "api")]
    #[error("Header error: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    /// GitHub API Error
    #[cfg(feature = "api")]
    #[error("GitHub API error: {0}")]
    ApiError(#[from] octocrab::Error),

    /// Zip Error
    #[cfg(feature = "zip")]
    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    /// Download Error
    #[error("Download error: {0}")]
    DownloadError(String),

    /// Generic Error
    #[error("Tool Cache error: {0}")]
    GenericError(String),
}
