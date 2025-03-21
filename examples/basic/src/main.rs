#![allow(dead_code)]

use anyhow::{Result, anyhow};
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Name of the Action
    name = "My Action",
    // Description of the Action
    description = "My Action Description"
)]
struct MyAction {
    /// Repository
    #[input(description = "Repository Name")]
    repository: String,

    /// GitHub Token
    #[input(description = "GitHub Token")]
    token: String,

    /// My Input
    #[input(description = "Mode", default = "false")]
    mode: bool,

    #[output(description = "Output Version")]
    version: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let action = MyAction::init()?;

    info!("Action :: {:?}", action);

    group!("Main Workflow");

    info!("Repository: `{}`", action.repository);
    info!("My Input Mode :: `{}`", action.mode);
    info!("My Output Version :: `{}`", action.version);

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
