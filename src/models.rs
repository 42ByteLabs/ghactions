//! # Models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Action YAML file structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionYML {
    /// Action Name
    pub name: Option<String>,
    /// Action Description
    pub description: Option<String>,
    /// Action Inputs
    pub inputs: HashMap<String, ActionInput>,
}

/// Action Input structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionInput {
    /// Input Description
    pub description: Option<String>,
    /// Input Required or not
    pub required: Option<bool>,
    /// Input Default value
    pub default: Option<String>,
}
impl ActionYML {
    /// Load the Action YAML file
    pub fn load_action(path: String) -> Result<ActionYML, Box<dyn std::error::Error>> {
        let fhandle = std::fs::File::open(path)?;
        let action_yml: ActionYML = serde_yaml::from_reader(fhandle)?;
        Ok(action_yml)
    }
}

/// Action Runs structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRuns {
    /// Action Name
    pub using: String,
    /// Docker Image
    pub image: Option<String>,
    /// Docker Arguments
    pub args: Option<Vec<String>>,
}
