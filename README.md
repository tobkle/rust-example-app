## Rust on Nails

Built with the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

## Setup

```sh
curl -L https://github.com/purton-tech/rust-on-nails/archive/main.tar.gz | \
  tar xvz --strip=2 rust-on-nails-main/nails-devcontainer/ \
  && rm devcontainer-template.json

```

Re-open VScode in Container

## Git

```sh
git init --initial-branch=main
```

## Workspace

Create a Cargo.toml

```toml
[workspace]
resolver = "3"

members = [
    "crates/*"
]
```

## Setup web-server

```sh
cargo new --vcs=none crates/web-server
cargo run
git add .
git commit -m "Initial Commit"
```