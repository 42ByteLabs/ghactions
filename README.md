# GitHub Actions library written in Rust

This is a Rust GitHub Actions library which should help those of us that write GitHub Actions in Rust.


## Usage

```rust
use ghactions::{info, debug, warn, error, group, groupend, errorf, setoutput};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut action = ghactions::init();

    if action.in_action() {
        info!("GitHub Action Name :: {}", &action.name.clone().unwrap_or_else(|| "N/A".to_string()));

        let repository = action.get("repository").unwrap();

        group!("Main Workflow");

        info!(" > Repository('{}')", repository);

        let client = action.client
            .unwrap();

        let owner = "GeekMasher";
        let repo = "ghactions";

        // https://github.com/softprops/hubcaps/blob/master/examples/releases.rs
        let latest = client.repo(owner, repo).releases().latest().await?;
        info!("{:#?}", latest);

        for r in client.repo(owner, repo).releases().list().await? {
            info!("  -> {}", r.name);
        }

        groupend!();
    }
    else {
        error!("Failed to load action.yml file");
    }
    Ok(())
}
```

*Note:* Do not use `.unwrap()` in a production Action.


## License 

This code is under the MIT License.

