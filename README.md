<!-- markdownlint-disable -->
<div align="center">
<h1>GitHub Actions library written in Rust</h1>

[![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)][github]
[![Crates.io Version](https://img.shields.io/crates/v/ghactions?style=for-the-badge)][crates-io]
[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/ghactions?style=for-the-badge)][crates-io]
[![GitHub Stars](https://img.shields.io/github/stars/42ByteLabs/ghactions?style=for-the-badge)][github]
[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)][license]

</div>
<!-- markdownlint-restore -->

This is a Rust GitHub Actions library which should help those of us that write
GitHub Actions in Rust.

## 📦 Install

Run the following command to add the library to your `Cargo.toml` file:

```bash
cargo add ghactions
```

## 📚 Features

- Easy to use
- Generate `action.yml` file automatically from code
- Validate GitHub Actions files
- Automatical input and output parsing
- [Octocrab][octocrab] support

## 🚀 Usage

Here is a simple example of how to use the library:

```rust
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    name = "My Action",
    description = "My Action Description",
)]
struct MyAction {
    /// My Input
    #[input()]
    mode: bool,

    // Output called `version`
    #[output()]
    version: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the Action
    let action = MyAction::init()?;

    info!("Action :: {:?}", action);

    group!("Main Workflow");

    info!("My Input Mode :: `{}`", action.mode);
    info!("My Output Version :: `{}`", action.version);

    groupend!();

    Ok(())
}
```

### Using Template (cargo-generate)

You can use the [cargo-generate](cargo-generate) tool to create a new GitHub Action project with the library.

```bash
cargo generate --git https://github.com/42ByteLabs/ghactions
```

### Using Octocrab

```rust
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
struct MyAction {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action = MyAction::init()?;

    group!("Octocrab");
    let octocrab = action.octocrab()?;

    // ... Do something...

    Ok(())
}
```

## 🦸 Support

Please create [GitHub Issues][github-issues] if there are bugs or feature requests.

This project uses [Semantic Versioning (v2)][semver] and with major releases, breaking changes will occur.

## 📓 License

This project is licensed under the terms of the MIT open source license.
Please refer to [MIT][license] for the full terms.

<!-- Resources -->
[license]: ./LICENSE
[semver]: https://semver.org/
[github]: https://github.com/42ByteLabs/ghactions
[github-issues]: https://github.com/42ByteLabs/ghactions/issues
[crates-io]: https://crates.io/crates/ghactions
[examples]: ./examples
[octocrab]: https://crates.io/crates/octocrab
[cargo-generate]: https://crates.io/crates/cargo-generate

