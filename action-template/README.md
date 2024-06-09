# 42ByteLabs Action Template

This is a template for creating new GitHub Actions.

## Instructions

0. Install Cargo Generate

```bash
cargo install cargo-generate
```

1. Generate a new GitHub Action using repository template

```bash 
cargo generate https://github.com/42ByteLabs/ghactions
```

2. Select the template and fill in the required information

3. Build the action

```bash
cargo build
```

### Docker

This template is primarily designed to be used with Docker.
The Dockerfile is already included in the template.
You can build the Docker image using the following command:

```bash
docker build -t ghactions .
```

The `actions/Dockerfile` is so you don't have to install anything and you pull the image from the GitHub Container Registry.
You can change this to whatever you want.

