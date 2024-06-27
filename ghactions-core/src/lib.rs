//! ghactions-core is a library that provides core functionality for GitHub Actions in Rust.
#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(missing_docs)]

#[cfg(feature = "log")]
extern crate log;

pub mod actions;
pub mod errors;
// pub mod ghaction;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "log")]
pub mod logging;
pub mod repository;

pub use crate::actions::models::{ActionInput, ActionRuns, ActionYML};
pub use crate::errors::ActionsError;
pub use crate::repository::reference::RepositoryReference;

#[cfg(feature = "cache")]
pub use cache::Cache;

/// Action Trait
pub trait ActionTrait {
    /// Parse the action input
    fn init() -> Result<Self, ActionsError>
    where
        Self: Sized;

    /// Get the action name
    fn name(&self) -> &str;

    /// Get the action description
    fn description(&self) -> &str;

    /// Get the input value for a provided key
    fn get_input(key: impl Into<String> + Copy) -> Result<String, ActionsError> {
        std::env::var(&key.into()).map_err(|_| ActionsError::InputError(key.into()))
    }

    /// Get the input value for a provided key as a boolean
    fn get_input_bool(key: impl Into<String> + Copy) -> Result<bool, ActionsError> {
        Self::get_input(key)?
            .parse::<bool>()
            .map_err(|_| ActionsError::InputTypeError(key.into(), "bool".into()))
    }

    /// Get the input value for a provided key as an integer
    fn get_input_int(key: impl Into<String> + Copy) -> Result<i32, ActionsError> {
        Self::get_input(key)?
            .parse::<i32>()
            .map_err(|_| ActionsError::InputTypeError(key.into(), "int".into()))
    }

    /// Get the input value for a provided key as a vector using a seperator
    fn get_input_vec(
        key: impl Into<String> + Copy,
        seperator: &str,
    ) -> Result<Vec<String>, ActionsError> {
        Ok(Self::get_input(key)?
            .split(seperator)
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
    }

    /// Set the output value for a provided key
    fn set_output(
        key: impl Into<String> + Copy,
        value: impl Into<String> + Copy,
    ) -> Result<(), ActionsError> {
        let key = key.into();
        let value = value.into();

        setoutput!(key, value);

        Ok(())
    }

    /// Get the Octocrab instance
    ///
    /// Uses the `GITHUB_API_URL` and `GITHUB_TOKEN` environment variable to create an Octocrab instance
    #[cfg(feature = "octocrab")]
    fn octocrab(&self) -> Result<octocrab::Octocrab, ActionsError> {
        #[cfg(feature = "log")]
        {
            log::debug!("Creating Octocrab instance");
            log::debug!("URL: {}", self.get_api_url());
        }

        match self.get_token() {
            Ok(token) => Ok(octocrab::Octocrab::builder()
                .base_uri(self.get_api_url())
                .map_err(|e| ActionsError::OctocrabError(e.to_string()))?
                .add_header(
                    http::header::ACCEPT,
                    "application/vnd.github.v3+json".to_string(),
                )
                .personal_token(token)
                .build()
                .map_err(|e| ActionsError::OctocrabError(e.to_string()))?),
            Err(_) => {
                #[cfg(feature = "log")]
                log::warn!("No GitHub Token provided");

                Ok(octocrab::Octocrab::builder()
                    .base_uri(self.get_api_url())
                    .map_err(|e| ActionsError::OctocrabError(e.to_string()))?
                    .add_header(
                        http::header::ACCEPT,
                        "application/vnd.github.v3+json".to_string(),
                    )
                    .build()
                    .map_err(|e| ActionsError::OctocrabError(e.to_string()))?)
            }
        }
    }

    /// GetHub Server URL (default: https://github.com)
    fn get_server_url(&self) -> String {
        Self::get_input("GITHUB_SERVER_URL").unwrap_or_else(|_| "https://github.com".into())
    }
    /// GitHub API URL (default: https://api.github.com)
    fn get_api_url(&self) -> String {
        Self::get_input("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".into())
    }
    /// GitHub GraphQL URL (default: https://api.github.com/graphql)
    fn get_graphql_url(&self) -> String {
        Self::get_input("GITHUB_GRAPHQL_URL")
            .unwrap_or_else(|_| "https://api.github.com/graphql".into())
    }

    /// Get the GitHub Token
    fn get_token(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_TOKEN")
    }
    /// Get the GitHub SHA
    fn get_sha(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_SHA")
    }
    /// Get the GitHub Ref (full)
    fn get_ref(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_REF")
    }
    /// Get the GitHub Ref Type
    fn get_ref_type(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_REF_TYPE")
    }
    /// Get the GitHub Ref Name
    fn get_ref_name(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_REF_NAME")
    }

    /// Get the GitHub Workflow Event Name
    fn get_event_name(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_EVENT_NAME")
    }

    /// Get the full GitHub Repository (owner/repo)
    fn get_repository(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_REPOSITORY")
    }
    /// Get the GitHub Repository owner name (org/user)
    fn get_repository_owner(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_REPOSITORY_OWNER").or_else(|_| {
            self.get_repository()
                .map(|r| r.split('/').collect::<Vec<&str>>()[0].to_string())
        })
    }
    /// Get the GitHub Repository name
    fn get_repository_name(&self) -> Result<String, ActionsError> {
        self.get_repository()
            .map(|r| r.split('/').collect::<Vec<&str>>()[1].to_string())
    }
    /// Get the GitHub Repository URL
    fn get_repository_url(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_REPOSITORYURL")
    }
    /// Get the Action Triggering Author
    fn get_actor(&self) -> Result<String, ActionsError> {
        Self::get_input("GITHUB_ACTOR")
    }
}
