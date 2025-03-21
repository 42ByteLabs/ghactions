//! Errors for the core library
use thiserror::Error;

/// Actions Error
#[derive(Error, Debug)]
pub enum ActionsError {
    /// Failed to load the environment
    #[error("Failed to load environment: `{0}`")]
    FailedLoading(String),

    /// Failed to get input value from environment
    #[error("Failed to get input value: `{0}`")]
    InputError(String),

    /// Input Type Error
    #[error("Input Type Error: `{0}` (Expected: `{1}`)")]
    InputTypeError(String, String),

    /// IO Error
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    /// Tool Cache Error
    #[cfg(feature = "toolcache")]
    #[error("Tool Cache Error: `{0}`")]
    ToolCacheError(String),

    /// Tool Not Found in Cache
    #[cfg(feature = "toolcache")]
    #[error("Tool Not Found in Cache: `{0}`")]
    ToolNotFound(String),

    /// Glob Error
    #[cfg(feature = "toolcache")]
    #[error("Glob Pattern Error")]
    PatternError(#[from] glob::PatternError),

    /// Octocrab Error
    #[cfg(feature = "octocrab")]
    #[error("Octocrab Error: `{0}`")]
    OctocrabError(String),

    /// Failed parsing the repository reference
    #[error("Unable to parse repo reference: `{0}`")]
    RepositoryReferenceError(String),

    /// IO Error
    #[error("IO Error: `{0}`")]
    IOError(String),

    /// Not Implemented
    #[error("Not Implemented")]
    NotImplemented,
}
