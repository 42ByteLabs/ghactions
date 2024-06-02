//! GHActions is a library to make it easier to write GitHub Actions in Rust.

use log::{debug, info, warn};
#[cfg(feature = "octocrab")]
use octocrab::Octocrab;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, env};

use crate::{models::ActionYML, GHActionError, RepositoryReference};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_environment_variables(prefix: &str) -> HashMap<String, String> {
    let mut list = HashMap::<String, String>::new();
    for (env_key, env_value) in env::vars() {
        if env_key.starts_with(prefix) {
            let fkey = format!("{}_", prefix);
            let new_key = env_key.replace(&fkey, "").to_lowercase();
            debug!("Loading `{}` Env Key: {}", prefix, new_key);
            list.insert(new_key, env_value);
        }
    }
    list
}

/// Sets the output of the Actions which can be used in subsequent Actions.
///
/// # Examples
///
/// ```
/// use ghactions::setoutput;
///
/// # fn foo() {
/// setoutput!("hello", "world");
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! setoutput {
    // setoutput!("name", "value")
    ($($arg:tt)+) => (log!($crate::Level::Info, "::set-output name={}::{}", $($arg)+))
}

/// GHAction automation to make your life easier when writing Rust
/// GitHub Actions.
///
/// # Examples
///
/// ```
/// # use anyhow::Result;
/// use ghactions::{GHAction, info};
///
/// #[tokio::main]
/// async fn main() {
///     let action = GHAction::new().unwrap();
///
///     // Name of your the Action
///     info!("GitHub Action Name :: {:?}", action.name);
/// }
///```
///
/// Note: Do not use `.unwrap()` in production Actions
///
#[derive(Debug)]
pub struct GHAction {
    /// Path of the Action YML File
    pub path: String,
    /// Repository Reference
    pub repository: RepositoryReference,

    #[cfg(feature = "octocrab")]
    /// GitHub Client (Octocrab)
    pub client: Option<Octocrab>,

    /// Action Name
    pub name: Option<String>,
    /// Action Description
    pub description: Option<String>,
    /// Action Inputs
    pub inputs: HashMap<String, String>,

    /// GitHub Environment Variables
    ///
    /// https://docs.github.com/en/actions/learn-github-actions/environment-variables
    pub github: HashMap<String, String>,
    /// Runner Environment Variables
    pub runner: HashMap<String, String>,
    /// If the Action is loaded
    pub loaded: bool,
}

impl GHAction {
    /// Create a new GHAction struct
    ///
    /// ```
    /// use ghactions::{GHAction, info};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut action = GHAction::new().unwrap();
    ///     info!("Action Name :: {}", action.name.unwrap_or_else(|| "N/A".to_string()));
    ///     info!("Action Path :: {}", action.path);
    /// }
    ///
    /// ```
    pub fn new() -> Result<Self, GHActionError> {
        debug!("Loading dotenv...");

        let action_path = GHAction::default_path();

        debug!("Action YML File :: {}", action_path);

        let github = load_environment_variables("GITHUB");

        // repository

        let repository: RepositoryReference = match github.get("repository") {
            Some(repo) => RepositoryReference::parse(repo).unwrap(),
            None => RepositoryReference::default(),
        };

        // Create the init action struct
        #[allow(unused_mut)]
        let mut action = GHAction {
            path: action_path,
            repository,
            #[cfg(feature = "octocrab")]
            client: None,
            name: None,
            description: None,
            inputs: load_environment_variables("INPUT"),
            github,
            runner: load_environment_variables("RUNNER"),
            loaded: false,
        };

        #[cfg(feature = "octocrab")]
        {
            // Octocrab magic
            let github_token: String = action.get_token().unwrap_or_default();
            let client_builder = Octocrab::builder().personal_token(github_token).build();

            action.client = match client_builder {
                Ok(c) => Some(c),
                Err(err) => {
                    warn!("Failed to load client: {}", err.to_string());
                    None
                }
            };
        }

        Ok(action)
    }

    fn default_path() -> String {
        let mut path = PathBuf::new();

        // If the environment variable
        if let Ok(p) = env::var("GITHUB_ACTION_PATH") {
            path = PathBuf::from(&p);
        } else if let Ok(p) = std::env::current_exe() {
            let mut exe_path = p;
            // Remove exe file name
            exe_path.pop();
        } else {
            debug!("Using relative path to working directory");
        }

        path.push("action.yml");

        if Path::new(&path).exists() {
            info!("Path Exists");
        }

        path.into_os_string()
            .into_string()
            .expect("Unable to create default Action path")
    }

    /// Check and get the GitHub token from the many locations it could be stored at
    ///
    /// 1. Environment Variable: `GITHUB_TOKEN`
    /// 2. Environment Variable: `ACTIONS_RUNTIME_TOKEN`
    /// 3. Actions Input: `token`
    ///
    /// ```
    /// use ghactions::GHAction;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut action = GHAction::new().unwrap();
    ///
    ///     println!("{:?}", action.get_token());
    /// }
    /// ```
    pub fn get_token(&mut self) -> Option<String> {
        // Env Var: GITHUB_TOKEN
        match self.github.get("token") {
            Some(t) => return Some(t.to_string()),
            None => {
                debug!("Failed to find token at GITHUB_TOKEN");
            }
        };
        // Env Var: ACTIONS_RUNTIME_TOKEN
        match std::env::var("ACTIONS_RUNTIME_TOKEN") {
            Ok(t) => return Some(t),
            Err(_err) => {
                debug!("Failed to find token at ACTIONS_RUNTIME_TOKEN");
            }
        };
        // Input `token`
        match self.inputs.get("token") {
            Some(t) => return Some(t.to_string()),
            None => {
                debug!("Failed to find token at INPUT_TOKEN");
            }
        }

        None
    }

    /// Check to see if there is an Action yaml file present
    ///
    /// ```
    /// use ghactions::{GHAction, info};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut action = GHAction::new().unwrap();
    ///     if action.in_action() {
    ///         info!("Action Name :: {}", &action.name.unwrap_or_else(|| "N/A".to_string()));
    ///     }
    /// }
    /// ```
    pub fn in_action(&mut self) -> bool {
        Path::new(&self.path).exists()
    }

    /// Set the Action YML Path (directory)
    ///
    /// ```
    /// # use ghactions::info;
    /// use ghactions::GHAction;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut action = GHAction::new().unwrap();
    ///
    ///     action.set_path(String::from("./subpath"));
    /// }
    /// ```
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }

    fn encode_envvar(prefix: &str, key: &str) -> String {
        let new_key: String = key.to_owned().replace('-', "_").to_uppercase();
        format!("{}_{}", prefix, &new_key)
    }

    /// Get an environment variable from the GitHub Action
    pub fn get(&mut self, key: &str) -> Option<String> {
        let new_key = key.to_lowercase();
        if self.github.contains_key(&new_key) {
            return Some(self.github.get(&new_key).unwrap().to_string());
        }
        if self.inputs.contains_key(&new_key) {
            return Some(self.inputs.get(&new_key).unwrap().to_string());
        }

        None
    }

    /// Gets an input passed into the Action using a key and pre-loaded inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use ghactions::GHAction;
    ///
    /// # fn foo() {
    /// let mut action = GHAction::new();
    ///
    /// # }
    /// ```
    pub fn get_input(&mut self, input: &str) -> Option<String> {
        let new_input = input.to_lowercase();
        if self.inputs.contains_key(&new_input) {
            debug!("Action input found: {}", &input);
            return Some(self.inputs.get(&new_input).unwrap().to_string());
        }
        None
    }

    /// Set an output for the Action
    pub fn set_output(&mut self, name: &str, value: &str) {
        setoutput!(name, value);
    }

    /// Load the Action YML file and set the name, description, and inputs
    pub fn load_actions_file(&mut self) -> &mut Self {
        info!("Loading Action file: {}", &self.path);

        match ActionYML::load_action(self.path.clone()) {
            Ok(action_yml) => {
                debug!("Found and loaded Actions YML file");

                self.name = action_yml.name;
                self.description = action_yml.description;

                for (key, _value) in action_yml.inputs.into_iter() {
                    let key_envvar = GHAction::encode_envvar("INPUT", &key);
                    match env::var(key_envvar) {
                        Ok(v) => {
                            debug!("Found key and envvar: {}", &key);
                            self.inputs.insert(key, v);
                        }
                        Err(_e) => {
                            warn!("Failed to find key: {}", &key);
                        }
                    }
                }
                self.loaded = true;
            }
            Err(e) => {
                warn!("Failed to load inputs: {}", e);
            }
        };

        self
    }
}
