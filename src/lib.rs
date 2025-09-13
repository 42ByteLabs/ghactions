#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(missing_docs)]

pub use ghactions_core::ActionTrait;
pub use ghactions_core::ActionsError;
pub use ghactions_core::logging::init_logger;
#[cfg(feature = "log")]
pub use ghactions_core::{errorf, group, groupend, setoutput};
pub use ghactions_derive::Actions;
#[cfg(feature = "toolcache")]
pub use ghactions_toolcache::{ToolCache, ToolCacheArch, ToolPlatform};

/// Prelude module to re-export the most commonly used types
pub mod prelude {
    // Derive Macros
    pub use ghactions_derive::Actions;

    // Traits
    pub use ghactions_core::ActionTrait;

    // Structs / Functions
    pub use ghactions_core::errors::ActionsError;

    #[cfg(feature = "log")]
    pub use ghactions_core::{errorf, group, groupend, setoutput};
    #[cfg(feature = "log")]
    pub use log::{debug, error, info, trace, warn};

    // Tool Cache
    #[cfg(feature = "toolcache")]
    pub use ghactions_toolcache::ToolCache;
}
