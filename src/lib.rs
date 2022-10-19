#![allow(dead_code)]
#![allow(unused_imports)]
pub mod models;
pub mod ghaction;
pub mod logging;
pub mod reporef;

pub use crate::ghaction::GHAction;
pub use crate::reporef::RepositoryReference;
pub use crate::logging::init_logger;


// Publicly re-exporting logging functions
pub use log::{info, warn, debug, error, log, Level};


#[derive(thiserror::Error, Debug, PartialEq)]
pub enum GHActionError {
    #[error("Failed to load enviroment: `{0}`")]
    FailedLoading(String),

    #[error("Unable to parse repo reference: `{0}`")]
    RepositoryReferenceError(String), 
}

/// Initialise the GitHub Action by using the `init()` functions
///
/// ```
/// use log::info;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
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


#[cfg(test)]
mod tests {

}
