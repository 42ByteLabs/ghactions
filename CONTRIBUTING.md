# Contributing to GHActions

First of all, thank you for considering contributing to [ghactions][github] project!
We appreciate your interest and support.

## How to Contribute

You can contribute to this project in several ways:

- Reporting bugs
- Suggesting features
- Writing documentation
- Fixing bugs
- Implementing features

## Reporting Bugs

If you find a bug in the project, please [report it by creating an issue on the GitHub repository][github-issues].

## Suggesting Features

If you have an idea for a new feature or improvement, please [create an issue on the GitHub repository][github-issues] to discuss it with the community.

## Writing Documentation

If you find any part of the documentation unclear or incomplete, please feel free to [create an pull request for the GitHub repository][github] with your suggestions.

## Writing Code

### Building the Project

To build the project, you need to have [Rust][rust] installed.

```bash
cargo build --workspace
```

### Testing

We use [`cargo test`][cargo-test] to run tests. You can run all tests with the following command:

```bash
cargo test --workspace
```

You can also run the examples with the following command:

```bash
cargo run --example <example_name>
```

### Linting

All code should be linted with `rustfmt` and `clippy`. You can run both with the following command:

```bash
cargo fmt && cargo clippy
```

## Resources


<!-- Resources -->
[github]: https://github.com/42ByteLabs/ghactions
[github-issues]: https://github.com/42ByteLabs/ghactions/issues
[rust]: https://www.rust-lang.org/

