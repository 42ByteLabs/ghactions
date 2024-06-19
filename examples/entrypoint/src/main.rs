#![allow(dead_code)]

use anyhow::Result;
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Name of the Action
    name = "Advanced Compile Action",
    // Description of the Action
    description = "Advanced Compile Action Description",
    // Setting the path to the action.yml file
    //
    // If the `generate` feature is enabled, the action.yml file will be generated
    // dynamically based on the struct fields
    path = "./examples/entrypoint/action.yml",
    // Setting the entrypoint for the action
    entrypoint = "./examples/entrypoint/entrypoint.sh",
)]
struct MyAction {
    #[input(description = "Crate names (comma separated)", split = ",")]
    crates: Vec<String>,

    #[output(description = "Output Version")]
    version: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let action = MyAction::init()?;

    println!("Crates: {:?}", action.crates);

    Ok(())
}
