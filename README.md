# GitHub Actions library written in Rust

[![Rust](https://github.com/GeekMasher/rust-templates/actions/workflows/build.yml/badge.svg)](https://github.com/GeekMasher/rust-templates/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/d/ghactions.svg)](https://crates.io/crates/ghactions)
[![Documentation](https://docs.rs/ghactions/badge.svg)](https://docs.rs/ghactions/)

This is a Rust GitHub Actions library which should help those of us that write GitHub Actions in Rust.


## Usage

```rust
use ghactions::{info, debug, warn, error, group, groupend, errorf, setoutput};
use ghactions::reporef::RepositoryReference;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut action = ghactions::init();

    if ! action.in_action() {
        error!("Failed to load action.yml file");
        return Err(Error);
    }

    info!("GitHub Action Name :: {}", &action.name.clone().unwrap_or_else(|| "N/A".to_string()));

    group!("Main Workflow");

    info!("Repository: `{}`", action.repository.display());

    let client = action.client?;

    // https://github.com/softprops/hubcaps/blob/master/examples/releases.rs
    let latest = client.repo(owner, repo).releases().latest().await?;
    info!("{:#?}", latest);

    for r in client.repo(owner, repo).releases().list().await? {
        info!("  -> {}", r.name);
    }

    groupend!();

    Ok(())
}
```


## License 

This code is under the MIT License.

