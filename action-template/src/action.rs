use ghactions::prelude::*;

#[derive(Actions, Debug)]
#[action(
    name = "{{action_name}}",
    description = "{{description}}",
    path = "./action.yml"
)]
pub struct MyAction {
    // Inputs & Outputs go here
}
