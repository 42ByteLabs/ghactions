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
    #[error("Unable to parse repo reference: `{0}`")]
    RepositoryReferenceError(String), 
}

/// Initialise the GitHub Action by using the `init()` functions
///
/// ```
/// use log::info;
///
/// # fn main() {
/// let mut action = ghactions::init();
///
/// info!("GitHub Action Name :: {}", &action.name.unwrap_or_else(|| "N/A".to_string()));
/// # }
/// ```
pub fn init() -> GHAction {
    init_logger().init();
    debug!("Debugging is enabled!");

    let mut action = GHAction::new();
    // Load the Action file
    action.load_actions_file();

    action
}


#[cfg(test)]
mod tests {

}
