#![allow(dead_code)]

use anyhow::{Result, anyhow};
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Name of the Action
    name = "My Action",
    // Description of the Action
    description = "My Action Description",
    // Author
    author = "GeekMasher",
    // Branding icon
    icon = "star",
    // Branding Color
    color = "green",
    // Setting the path to the action.yml file
    //
    // If the `generate` feature is enabled, the action.yml file will be generated
    // dynamically based on the struct fields
    path = "./examples/advanced/action.yml",
    // Set the Docker Image for the Action
    image = "./examples/advanced/Dockerfile",
)]
struct MyAction {
    /// Repository
    #[input(
        // Rename the input field
        name = "repo",
        description = "Repository Name",
        default = "${{ github.repository }}"
    )]
    repository: String,

    /// GitHub Token
    #[input(description = "GitHub Token", default = "${{ github.token }}")]
    token: String,

    /// My Input
    #[input(description = "Mode", default = "false")]
    mode: bool,

    #[input(description = "Crate names (comma separated)", split = ",")]
    crates: Vec<String>,

    #[output(description = "Output Version")]
    version: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut action = MyAction::init()?;

    info!("Action :: {:?}", action);

    group!("Main Workflow");

    info!("Repository: `{}`", action.repository);
    info!("My Input Mode :: `{}`", action.mode);
    info!("My Input Crates :: `{:?}`", action.crates);

    action.set_version("1.0.0");

    info!("My Output Version :: `{}`", action.version);
    assert_eq!(action.version, "1.0.0");

    group!("Octocrab");
    let octocrab = action.octocrab()?;

    let repository = octocrab
        .repos(
            action.get_repository_owner()?,
            action.get_repository_name()?,
        )
        .get()
        .await?;

    info!(
        "{} - {}",
        repository
            .full_name
            .ok_or(anyhow!("Failed to get full name"))?,
        repository.url.to_string()
    );

    groupend!();

    Ok(())
}
