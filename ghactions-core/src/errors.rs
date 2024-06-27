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

    /// Cache Error
    #[cfg(feature = "cache")]
    #[error("Cache Error: `{0}`")]
    CacheError(String),

    /// Input Type Error
    #[error("Input Type Error: `{0}` (Expected: `{1}`)")]
    InputTypeError(String, String),

    /// Octocrab Error
    #[cfg(feature = "octocrab")]
    #[error("Octocrab Error: `{0}`")]
    OctocrabError(String),

    /// Failed parsing the repository reference
    #[error("Unable to parse repo reference: `{0}`")]
    RepositoryReferenceError(String),

    /// IO Error
    #[error("IO Error: `{0}`")]
    IOError(#[from] std::io::Error),

    /// YAML Serialization Error
    #[error("YAML Serialization: `{0}`")]
    YAMLSerializationError(#[from] serde_yaml::Error),

    /// Glob Error
    #[cfg(feature = "cache")]
    #[error("Glob Error: `{0}`")]
    GlobError(#[from] glob::GlobError),

    /// Not Implemented
    #[error("Not Implemented")]
    NotImplemented,
}
