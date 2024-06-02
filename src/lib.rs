//! # GitHub Actions
//!
//! `ghactions` is a library to help with the development of GitHub Actions in Rust.
//!
//! ## Features
//!
//! - [x] Load the Action file
//! - [x] Logging utilities
//!
//! ## Basic Usage
//!
//! ```no_run
//! # use anyhow::{anyhow, Result};
//! use ghactions::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let action = ghactions::init()?;
//!
//!     info!(
//!         "GitHub Action Name :: {}",
//!         &action.name.clone().unwrap_or_else(|| "N/A".to_string())
//!     );
//!
//!     group!("Main Workflow");
//!
//!     info!("Repository: `{}`", action.repository.display());
//!
//!     let client = action.client.ok_or(anyhow!("Failed to load client"))?;
//!
//!     // https://github.com/XAMPPRocky/octocrab
//!     let repository = client
//!         .repos(action.repository.owner, action.repository.name)
//!         .get()
//!         .await?;
//!
//!     info!(
//!         "{} - {}",
//!         repository
//!             .full_name
//!             .ok_or(anyhow!("Failed to get full name"))?,
//!         repository.url.to_string()
//!     );
//!
//!     groupend!();
//!     let mut action = GHAction::new()?;
//!
//!     println!("Action Name :: {}", action.name.unwrap_or_else(|| "N/A".to_string()));
//!
//!     Ok(())
//! }
//! ```
//!
#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(missing_docs)]

pub mod ghaction;

#[cfg(feature = "log")]
pub mod logging;
pub mod models;
pub mod reporef;

pub use crate::ghaction::GHAction;
pub use crate::logging::init_logger;
pub use crate::reporef::RepositoryReference;

// Publicly re-exporting logging functions
pub use log::{debug, error, info, log, warn, Level};

/// GHActionError is a custom error type for the GitHub Action
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum GHActionError {
    /// Failed to load the environment
    #[error("Failed to load environment: `{0}`")]
    FailedLoading(String),

    /// Failed parsing the repository reference
    #[error("Unable to parse repo reference: `{0}`")]
    RepositoryReferenceError(String),
}

/// Initialise the GitHub Action by using the `init()` functions
///
/// ```
/// use log::info;
/// use anyhow::Result;
///
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let mut action = ghactions::init()?;
///
///     info!("GitHub Action Name :: {}", &action.name.unwrap_or_else(|| "N/A".to_string()));
///
///     Ok(())
/// }
/// ```
pub fn init() -> Result<GHAction, GHActionError> {
    init_logger().init();
    debug!("Debugging is enabled!");

    let mut action = match GHAction::new() {
        Ok(a) => a,
        Err(err) => {
            error!("{}", err.to_string());
            return Err(err);
        }
    };
    // Load the Action file
    action.load_actions_file();

    Ok(action)
}

/// Prelude module to re-export the most commonly used types
pub mod prelude {
    pub use crate::ghaction::GHAction;
    pub use crate::logging::init_logger;
    pub use crate::reporef::RepositoryReference;

    // Macros
    #[cfg(feature = "log")]
    pub use crate::{debug, error, group, groupend, info, warn};
}
