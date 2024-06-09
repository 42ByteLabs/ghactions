use ghactions::prelude::*;

use anyhow::Result;
use log::info;

mod action;


#[tokio::main]
async fn main() -> Result<()> {
    let action = action::MyAction::init()?;
    info!("Starting action: {}", action.name());

    let octocrab = action.octocrab()?;

    // Your code goes here

    Ok(())
}
