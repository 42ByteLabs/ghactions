use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionYML {
    pub name: Option<String>,
    pub description: Option<String>,

    pub inputs: HashMap<String, ActionInput>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionInput {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub default: Option<String>,
}
impl ActionYML {
    pub fn load_action(path: String) -> Result<ActionYML, Box<dyn std::error::Error>> {
        let fhandle = std::fs::File::open(path)?;
        let action_yml: ActionYML = serde_yaml::from_reader(fhandle)?;
        Ok(action_yml)
    }
}




