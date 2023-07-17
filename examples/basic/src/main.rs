use anyhow::{anyhow, Result};
use ghactions::{group, groupend, info};

#[tokio::main]
async fn main() -> Result<()> {
    let action = ghactions::init()?;

    info!(
        "GitHub Action Name :: {}",
        &action.name.clone().unwrap_or_else(|| "N/A".to_string())
    );

    group!("Main Workflow");

    info!("Repository: `{}`", action.repository.display());

    let client = action.client.ok_or(anyhow!("Failed to load client"))?;

    // https://github.com/XAMPPRocky/octocrab
    let repository = client
        .repos(action.repository.owner, action.repository.name)
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
