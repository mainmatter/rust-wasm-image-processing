# Rust & WebAssembly Image Processing Workshop

You've heard about Rust, but you never had the chance to try it out?\
This course is for you!

You'll learn Rust by writing an image processing application.\
You'll go from knowing nothing about Rust to being able to start
writing your own programs, one exercise at a time.

> [!NOTE]
> This course has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [our portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or
> training!

## Getting Started

Go through the [Dev Setup](#dev-setup) below and then run `./serve.sh` to open the book and start the backend. The serve script will automatically build your Rust code, and rerun it when you change things.

With the frontend running navigate to `http://localhost:3000` in order to get started with the first exercise!

Open the workshop at [http://localhost:3000/](http://localhost:3000/).

## Dev Setup

There are three main supported ways:

### Dev Container

A pre-made dev container is available. (We especially recommend this if your editor already supports it, e.g. VS Code.)

### Manual Setup

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

### Nix Flake

If you are already comfortable with using Nix flakes, one is available at `./flake.nix`.

## Solutions

You can find the solutions to the exercises in
the [`solutions` branch](https://github.com/mainmatter/rust-wasm-image-processing/tree/solutions) of this repository.

## License

Copyright © 2026- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).
