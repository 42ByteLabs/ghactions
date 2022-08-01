# GitHub Actions library written in Rust

This is a Rust GitHub Actions library which should help those of us that write GitHub Actions in Rust.

## Installing

**Cargo.toml**

```toml
[dependencies]
gh_actions = "0.0.2"
```


## Usage

```rust
use gh_actions::GHAction;

fn main() {
let action = GHAction::new();

if action.in_action() {
    // Name of your the Action
    let action_name = action.name.unwrap();

    println!(action_name);

    // github.com or Enterprise Server
    let api_url = action.get("api_url")
        .unwrap();

    // Get Actions Input
    let username = action.get_input("username")
        .unwrap();

    // Using the Hubcaps client
    let client = action.client
        .unwrap();

    let repo = client.repo("GeekMasher", "gh_actions");

}
```

*Note:* Do not use `.unwrap()` in a production Action.


## License 

This code is under the MIT License.

