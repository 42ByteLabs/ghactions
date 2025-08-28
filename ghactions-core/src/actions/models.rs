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

/// Action Mode
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionMode {
    /// Default Mode
    #[default]
    Default,
    /// Container/Docker Mode
    Container,
    /// Installer Mode
    Installer,
    /// Entrypoint Mode
    Entrypoint,
    /// Custom Composite Action
    CustomComposite,
}

/// Action YAML file structure
///
/// https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionYML {
    /// Action Mode
    #[serde(skip)]
    pub mode: ActionMode,

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
            mode: ActionMode::Default,
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
    pub fn set_container_image(&mut self, image: String) {
        self.runs.using = ActionRunUsing::Docker;
        self.runs.image = Some(image);
        self.runs.steps = None;
        // Docker based action doesn't need to set the output value step id
        self.output_value_step_id = None;
    }

    /// This mode uses a composite action with `gh` cli to install the action
    /// on the runner.
    ///
    /// https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/accessing-contextual-information-about-workflow-runs#github-context
    pub fn add_installer_step(&mut self) {
        if self.runs.steps.is_none() {
            self.runs.steps = Some(vec![]);
        }

        let binary_name =
            std::env::var("CARGO_BIN_NAME").unwrap_or_else(|_| "${{ github.action }}".to_string());

        let env = IndexMap::from([
            (
                "ACTION_REPOSITORY".to_string(),
                "${{ github.action_repository }}".to_string(),
            ),
            (
                "ACTION_REF".to_string(),
                "${{ github.action_ref }}".to_string(),
            ),
            ("BINARY_NAME".to_string(), binary_name.to_string()),
            ("GH_TOKEN".to_string(), "${{ github.token }}".to_string()),
            ("RUNNER_OS".to_string(), "${{ runner.os }}".to_string()),
            ("RUNNER_ARCH".to_string(), "${{ runner.arch }}".to_string()),
        ]);

        if let Some(ref mut steps) = self.runs.steps {
            // Linux / MacOS
            steps.push(ActionRunStep {
                name: Some("Install the Action".to_string()),
                id: Some("install-action".to_string()),
                shell: Some("bash".to_string()),
                condition: Some("${{ runner.os == 'Linux' || runner.os == 'macOS' }}".to_string()),
                env: Some(env.clone()),
                run: Some(include_str!("installer.sh").to_string()),
            });
            // TODO: Add Windows support
        }
    }

    /// Add a custom installer script to the Action
    pub fn add_script(&mut self, script: &str, id: Option<&str>) {
        if self.runs.steps.is_none() {
            self.runs.steps = Some(vec![]);
        }

        if let Some(ref mut steps) = self.runs.steps {
            // Add script inline in the step
            steps.push(ActionRunStep {
                id: id.map(|s| s.to_string()),
                name: Some("Installing Action".to_string()),
                shell: Some("bash".to_string()),
                run: Some(script.to_string()),
                ..Default::default()
            });
        }
    }

    /// Add run step to the Action
    pub fn add_script_run(&mut self) {
        if self.runs.steps.is_none() {
            self.runs.steps = Some(vec![]);
        }

        if let Some(ref mut steps) = self.runs.steps {
            self.output_value_step_id = Some("cargo-run".to_string());

            let binary_name = std::env::var("CARGO_BIN_NAME")
                .unwrap_or_else(|_| "${{ github.action }}".to_string());
            let script = format!("set -e\n{}", binary_name);
            // Add script inline in the step
            steps.push(ActionRunStep {
                name: Some("Run the Action".to_string()),
                id: Some("cargo-run".to_string()),
                shell: Some("bash".to_string()),
                run: Some(script.to_string()),
                ..Default::default()
            });
        }
    }

    /// Add cargo install step
    pub fn add_cargo_install_step(&mut self, binary_name: &str) {
        if self.runs.steps.is_none() {
            self.runs.steps = Some(vec![]);
        }

        if let Some(ref mut steps) = self.runs.steps {
            let cmd = if binary_name == "." {
                "cargo install --path .".to_string()
            } else {
                format!("cargo install \"{binary_name}\"")
            };

            steps.push(ActionRunStep {
                name: Some("Cargo Install".to_string()),
                shell: Some("bash".to_string()),
                run: Some(cmd),
                ..Default::default()
            });
        }
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
                std::fs::create_dir_all(parent)
                    .map_err(|err| ActionsError::IOError(err.to_string()))?;
            }

            let mut content = String::new();
            content.push_str("# This file is generated by ghactions\n");
            if self.mode == ActionMode::CustomComposite {
                content.push_str(
                    "# `ghactions` is generating all parts but not composite action steps\n",
                );
            } else {
                content.push_str(
                "# Do not edit this file manually unless you disable the `generate` feature.\n\n",
            );
            }
            content.push_str(
                serde_yaml::to_string(self)
                    .map_err(|err| ActionsError::IOError(err.to_string()))?
                    .as_str(),
            );

            // Create or Open the file
            let mut fhandle = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .map_err(|err| ActionsError::IOError(err.to_string()))?;
            fhandle
                .write_all(content.as_bytes())
                .map_err(|err| ActionsError::IOError(err.to_string()))?;

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
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionBranding {
    /// Color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// Action Runs structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionRuns {
    /// Action Name
    pub using: ActionRunUsing,

    /// Container Image (container actions only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
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
            steps: None,
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
    /// Step Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Step ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Run if condition
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// Environment Variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<IndexMap<String, String>>,

    /// Shell to use (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    /// Run command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,
}
