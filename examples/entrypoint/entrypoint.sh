#!/bin/bash
# This is a sample entrypoint script that can be used to start the Action.

# Check if Rust / Cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Rust / Cargo is not installed. Please install Rust / Cargo to proceed."
    exit 1
fi

# Compile and run the Rust program
cargo run --release --example entrypoint -F generate,dotenvy

