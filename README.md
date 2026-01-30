# Contentful Wasm Workshop

## Dev setup

There are three main supported ways:

### Dev container

A pre-maid dev container is available. (We especially recommend this if your editor already supports it, e.g. VS Code.)

### Manual setup

1. Install Rustup as described here https://rustup.rs/
1. Add the Wasm target, `rustup target add wasm32-unknown-unknown`
1. Install mdbook, `cargo install --locked mdbook@0.5.2`
1. Install wasm-pack, `cargo install --locked wasm-pack@0.14.0`
1. Install watchexec, `cargo install --locked watchexec-cli@2.3.3`

### Nix flake

If you are already comfortable with using Nix flakes, one is available at `./flake.nix`.
