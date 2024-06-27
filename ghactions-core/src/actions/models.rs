//! # Models

use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    io::Write,
    os::unix::fs::FileExt,
    path::PathBuf,
};

use crate::ActionsError;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Action Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Action Author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Action Branding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<ActionBranding>,

    /// Action Inputs
    pub inputs: IndexMap<String, ActionInput>,
    /// Action Outputs
    pub outputs: IndexMap<String, ActionOutput>,
    /// Output Value Step ID
    #[serde(skip)]
    pub output_value_step_id: Option<String>,

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
            inputs: IndexMap::new(),
            outputs: IndexMap::new(),
            output_value_step_id: Some("cargo-run".to_string()),
            runs: ActionRuns::default(),
        }
    }
}

impl ActionYML {
    /// Set the Action to a Container Image based Action
    pub fn set_container_image(&mut self, image: PathBuf) {
        self.runs.using = ActionRunUsing::Docker;
        self.runs.image = Some(image);
        self.runs.steps = None;
        // Docker based action doesn't need to set the output value step id
        self.output_value_step_id = None;
    }

    /// Load the Action YAML file
    pub fn load_action(path: String) -> Result<ActionYML, Box<dyn std::error::Error>> {
        let fhandle = std::fs::File::open(&path)?;
        let mut action_yml: ActionYML = serde_yaml::from_reader(fhandle)?;
        action_yml.path = Some(PathBuf::from(path.clone()));
        Ok(action_yml)
    }

    /// Write the Action YAML file
    pub fn write(&self) -> Result<PathBuf, ActionsError> {
        if let Some(ref path) = self.path {
            if !path.exists() {
                let parent = path.parent().unwrap();
                std::fs::create_dir_all(parent)?;
            }

            let mut content = String::new();
            content.push_str("# This file is generated by ghactions\n");
            content.push_str(
                "# Do not edit this file manually unless you disable the `generate` feature.\n\n",
            );
            content.push_str(serde_yaml::to_string(self)?.as_str());

            // Create or Open the file
            let mut fhandle = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;

            fhandle.write_all(content.as_bytes())?;

            Ok(path.clone())
        } else {
            Err(ActionsError::NotImplemented)
        }
    }
}

/// Action Input structure
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionInput {
    /// [internal] Action Field Name
    #[serde(skip)]
    pub action_name: String,
    /// [internal] Struct Field Name
    #[serde(skip)]
    pub field_name: String,
    /// [internal] Input Type
    #[serde(skip)]
    pub r#type: String,

    /// Input Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input Required or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Input Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// Deprecation Message
    #[serde(rename = "deprecationMessage", skip_serializing_if = "Option::is_none")]
    pub deprecation_message: Option<String>,

    // Other internal fields
    /// Separator
    #[serde(skip)]
    pub separator: Option<String>,
}

/// Action Output structure
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionOutput {
    /// Output Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Output Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
    pub using: ActionRunUsing,

    /// Container Image (container actions only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<PathBuf>,
    /// Arguments (container actions only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Steps (composite actions only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<ActionRunStep>>,
}

impl Default for ActionRuns {
    fn default() -> Self {
        Self {
            using: ActionRunUsing::Composite,
            image: None,
            args: None,
            steps: Some(default_composite_steps()),
        }
    }
}

fn default_composite_steps() -> Vec<ActionRunStep> {
    // Binary Name
    let binary_name = std::env::var("CARGO_BIN_NAME").unwrap_or_else(|_| "action".to_string());
    vec![
        // Step 1 - Checking for Cargo/Rust (needs to be installed by the user)
        // ActionRunStep {
        //     name: Some("Checking for Cargo/Rust".to_string()),
        //     shell: Some("bash".to_string()),
        //     run: Some("".to_string()),
        //     ..Default::default()
        // },
        // Step 2 - Compile the Action
        ActionRunStep {
            name: Some("Compile / Install the Action binary".to_string()),
            shell: Some("bash".to_string()),
            run: Some("set -e\ncargo install --path \"${{ github.action_path }}\"".to_string()),
            ..Default::default()
        },
        // Step 3 - Run the Action
        ActionRunStep {
            id: Some("cargo-run".to_string()),
            name: Some("Run the Action".to_string()),
            shell: Some("bash".to_string()),
            run: Some(format!("set -e\n{}", binary_name)),
            ..Default::default()
        },
    ]
}

/// Action Run Using Enum
#[derive(Debug, PartialEq, Deserialize)]
pub enum ActionRunUsing {
    /// Docker / Container Image
    Docker,
    /// Composite Action
    Composite,
}

impl From<&str> for ActionRunUsing {
    fn from(value: &str) -> Self {
        match value {
            "docker" => ActionRunUsing::Docker,
            "composite" => ActionRunUsing::Composite,
            _ => ActionRunUsing::Composite,
        }
    }
}

impl From<String> for ActionRunUsing {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl Serialize for ActionRunUsing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ActionRunUsing::Docker => serializer.serialize_str("docker"),
            ActionRunUsing::Composite => serializer.serialize_str("composite"),
        }
    }
}

/// Action Run Step
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ActionRunStep {
    /// Step ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Step Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Shell to use (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    /// Run command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,

    /// Environment Variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}
