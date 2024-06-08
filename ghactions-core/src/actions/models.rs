//! # Models

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const GHACTIONS_ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Action YAML file structure
///
/// https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionYML {
    /// Action Path
    #[serde(skip)]
    pub path: Option<PathBuf>,

    /// Action Name
    pub name: Option<String>,
    /// Action Description
    pub description: Option<String>,
    /// Action Author
    pub author: Option<String>,

    /// Action Branding
    pub branding: Option<ActionBranding>,

    /// Action Inputs
    pub inputs: HashMap<String, ActionInput>,
    /// Action Outputs
    pub outputs: HashMap<String, ActionOutput>,

    /// Action Runs
    pub runs: ActionRuns,
}

impl Default for ActionYML {
    fn default() -> Self {
        ActionYML {
            path: None,
            name: Some(env!("CARGO_PKG_NAME").to_string()),
            description: None,
            author: None,
            branding: None,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            runs: ActionRuns::default(),
        }
    }
}

impl ActionYML {
    /// Load the Action YAML file
    pub fn load_action(path: String) -> Result<ActionYML, Box<dyn std::error::Error>> {
        let fhandle = std::fs::File::open(&path)?;
        let mut action_yml: ActionYML = serde_yaml::from_reader(fhandle)?;
        action_yml.path = Some(PathBuf::from(path.clone()));
        Ok(action_yml)
    }

    /// Write the Action YAML file
    pub fn write(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(ref path) = self.path {
            if !path.exists() {
                let parent = path.parent().unwrap();
                std::fs::create_dir_all(parent)?;
            }
            Ok(path.clone())
        } else {
            let mut path = PathBuf::from(GHACTIONS_ROOT);
            path.push("action.yml");
            Ok(path)
        }
    }
}

/// Action Input structure
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionInput {
    /// Input Type
    #[serde(skip)]
    pub r#type: String,

    /// Input Description
    pub description: Option<String>,
    /// Input Required or not
    pub required: Option<bool>,
    /// Input Default value
    pub default: Option<String>,
    /// Deprecation Message
    #[serde(rename = "deprecationMessage", skip_serializing_if = "Option::is_none")]
    pub deprecation_message: Option<String>,
}

/// Action Output structure
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionOutput {
    /// Output Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Action Branding
///
/// https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions#branding
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionBranding {
    /// Action Color
    pub color: String,
    /// Action Icon
    pub icon: String,
}

/// Action Runs structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRuns {
    /// Action Name
    pub using: String,
    /// Docker Image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Docker Arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl Default for ActionRuns {
    fn default() -> Self {
        Self {
            using: String::from("docker"),
            image: Some(String::from("Dockerfile")),
            args: None,
        }
    }
}
