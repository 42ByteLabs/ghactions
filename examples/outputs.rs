use anyhow::Result;
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Name of the Action
    name = "My Action",
    // Description of the Action
    description = "My Action Description"
)]
struct Action {
    #[output(name = "my_output")]
    my_output: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut action = Action::init().unwrap();

    action.set_my_output("Hello World");

    Ok(())
}
