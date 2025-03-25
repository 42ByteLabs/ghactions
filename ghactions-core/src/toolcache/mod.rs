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
//!
//! let tool_cache = ToolCache::new();
//! let path = tool_cache.find("node", "14.17.0").await
//!     .expect("Failed to find tool in cache");
//!
//! println!("{:?}", path);
//! # Ok(())
//! # }
//! ```

pub mod arch;
pub mod archives;
pub mod cache;
pub mod tool;

pub use arch::ToolCacheArch;
pub use cache::ToolCache;
pub use tool::Tool;
