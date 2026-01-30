# Rust & WebAssembly Image Processing Workshop

## Dev setup

There are three main supported ways:

### Dev container

A pre-maid dev container is available. (We especially recommend this if your editor already supports it, e.g. VS Code.)

### Manual setup

- [**Rust**](https://www.rust-lang.org/tools/install).
  If `rustup` is already installed on your system, run `rustup update` (or another appropriate command depending on how you installed Rust on your system) to ensure you're running on the latest stable version.
- **Wasm32 target**
  To add the Rust `wasm32-unknown-unknown` target, run `rustup target add wasm32-unknown-unknown`.
- [**mdbook**](https://rust-lang.github.io/mdBook/guide/installation.html)
  To install `mdbook`, run `cargo install --locked mdbook@0.5.2`
- [**wasm-pack**](https://drager.github.io/wasm-pack/installer/)
  To install `wasm-pack`, run `cargo install --locked wasm-pack@0.14.0`
- [**watchexec**](https://github.com/watchexec/watchexec?tab=readme-ov-file#install)
  To install `watchexec`, run `cargo install --locked watchexec-cli@2.3.3`

### Nix flake

If you are already comfortable with using Nix flakes, one is available at `./flake.nix`.
