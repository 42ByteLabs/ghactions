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

`GHActions` is a Rust based library for helping users write amazing GitHub Actions in Rust!

## 📦 Install

Run the following command to add the library to your `Cargo.toml` file:

```bash
cargo add ghactions
```

## 📚 Features

- Easy to use
- Validate GitHub Actions files
- Automatic input and output parsing
- Generate `action.yml` file automatically from code
  - feature: `generate`
- [Octocrab][octocrab] support
  - feature: `octocrab`
- Actions ToolCache support
  - Locating tools in the toolcache
  - feature: `toolcache`

## 🚀 Usage

Here is a simple example of how to use the library:

```rust
use ghactions::prelude::*;

#[derive(Actions)]
struct MyAction {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the Action
    let action = MyAction::init()?;

    group!("Main Workflow");

    // Do something...

    Ok(())
}
```

### Using Template (cargo-generate)

You can use the [cargo-generate](cargo-generate) tool to create a new GitHub Action project with the library.

```bash
cargo generate --git https://github.com/42ByteLabs/ghactions
```

### Advance Usage

Another feature of `ghactions` is the ability to automatically parse inputs and outputs from the action.

```rust
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Action Name
    name = "My Action",
    // Action Description
    description = "My Action Description",
)]
struct MyAction {
    /// My Input
    #[input(
        // Change the name of the input from `my_mode` to `mode`
        name = "mode",
        // Input Description
        description = "My Input Description",
        // Default Value
        default = "default"
    )]
    my_mode: String,

    // Automatical type conversion
    #[input(
        // Input Description
        description = "My Input Description",
        default = "42",
    )]
    my_int: i32,

    // Multiple Inputs
    #[input(
        // Input Description
        description = "My Second Input Description",
        // Automatically split the input by `,`
        split = ",",
    )]
    mutiple: Vec<String>,

    // Output called `version`
    #[output(
        // Output Description
        description = "My Output Description",
    )]
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the Action
    let action = MyAction::init()?;

    group!("Main Workflow");
    info!("Action :: {:?}", action);

    info!("My Input Mode :: `{}`", action.my_mode);
    info!("My Multiple Input :: `{:?}`", action.mutiple);
    info!("My Output Version :: `{}`", action.version);

    groupend!();

    // Using the toolcache feature
    group!("ToolCache");
    
    // Load the ToolCache from the `examples/toolcache` folder
    let toolcache = ToolCache::from("examples/toolcache");
    // Find the Node.js 18.4.0 tool
    let tool = toolcache.find("node", "18.4.0").await?;
    info!("Tool :: {:?}", tool);

    groupend!();

    // There are 3 ways to set the output
    group!("Set Outputs");

    // Using the dynamically name Action method
    action.output_version("1.0.0");
    // Using the `set_output` method
    MyAction::set_output("version", "1.0.0")?;
    // Or the Macro `setoutput!` directly
    setoutput!("version", "1.0.0");

    groupend!();

    Ok(())
}
```

### Generating the `action.yml` file

The `generate` feature will allow you to generate the `action.yml` file from the code.
There are 3 ways to generate the `action.yml` file:

- [`image` mode](./examples/advanced): 
  - This will generate the `action.yml` file with a container `image`.
  - Image can be a path to a Dockerfile (e.g. `./Dockerfile`).
  - Image can be a docker image using the `docker://` prefix.
- [`entrypoint` mode](./examples/entrypoint):
  - This will generate a composite Action with an entrypoint script.
- [`installer` mode](./examples/installer):
  - This will generate a composite Action with an installer script.
  - Only supports Linux and macOS runners are supported.

```rust no_run
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Container / Docker file to supply the Action
    image = "./examples/advanced/Dockerfile",
    // [Optional] Supply a entrypoint for the container image
    entrypoint = "./examples/entrypoint/entrypoint.sh",
)]
struct ContainerAction {
    // ...
}

#[derive(Actions, Debug, Clone)]
#[action(
    // Point to a Docker image on DockerHub or GitHub Container Registry
    image = "docker://ghcr.io/42bytelabs/ghactions:latest", 
)]
struct ContainerSocketAction {
    // ...
}

#[derive(Actions, Debug, Clone)]
#[action(
    // Entrypoint is a Composite Action with an entrypoint script
    entrypoint = "./examples/entrypoint/entrypoint.sh",
)]
struct EntrypointAction {
    // ...
}

#[derive(Actions, Debug, Clone)]
#[action(
    // Composite Action with an installer script
    installer
)]
struct InstallerAction {
    // ...
}

```

At build time, the `action.yml` file will be generated with the following content.

### Using Octocrab

Enabling the `octocrab` feature will allow you to use the [Octocrab][octocrab] library.

```rust
use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
struct MyAction {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action = MyAction::init()?;

    group!("Octocrab");
    // Automatically setup Octocrab with the GitHub Instance and Token
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

