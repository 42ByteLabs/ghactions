#![allow(dead_code)]
#![allow(unused_imports)]
pub mod models;
pub mod ghaction;
pub mod logging;

// 
pub use ghaction::GHAction;
pub use logging::init_logger;

// Publicly re-exporting logging functions
pub use log::{info, warn, debug, error, log, Level};


/// Initialise the GitHub Action by using the `init()` functions
///
/// ```
/// use log::info;
///
/// # fn main() {
/// let mut action = ghactions::init();
///
/// if action.in_action() {
///     info!("Running Action...");
/// }
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
