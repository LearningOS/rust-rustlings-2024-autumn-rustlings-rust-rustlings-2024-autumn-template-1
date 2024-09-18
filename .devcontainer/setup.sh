#!/bin/bash
curl https://sh.rustup.rs -sSf | sh -s -- -y

# Update current shell environment variables after install to find rustup
. "$HOME/.cargo/env"
rustup install stable
cargo install --force --path .
