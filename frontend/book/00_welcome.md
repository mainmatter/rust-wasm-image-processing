# Welcome

Welcome to the **Rust & WebAssembly Image Processing Workshop**!

This hands-on workshop teaches you Rust fundamentals through practical image transformation exercises.
You'll learn Rust's syntax, type system, and key concepts while building real image processing functions
that run both on the server and in the browser via WebAssembly.

We don't assume any prior knowledge of Rust, but do assume you are at least familiar with web development.
You'll get a taste for Rust in small, manageable steps.

By the end of the workshop, you will have completed 4 exercises covering functions, pattern matching,
collections, and parallel iteration—giving you a solid foundation for working with Rust and WebAssembly.

## Methodology

This workshop is based on the "learn by doing" principle.\
It has been designed to be interactive and hands-on.

[Mainmatter](https://mainmatter.com/rust-consulting/) developed this workshop
to be delivered in a classroom setting, each attendee advances
through the lessons at their own pace, with an experienced instructor providing
guidance, answering questions and diving deeper into the topics as needed.\
If you'd like to organise a private session for your company, please [get in touch](https://mainmatter.com/contact/).

You can also take the workshops on your own, but we recommend you find a friend or
a mentor to help you along the way should you get stuck. You can
find solutions for all the exercises in the
[`solutions`](https://github.com/mainmatter/rust-wasm-image-processing/tree/solutions) branch of the GitHub repository.

## Structure

On the left side of the screen, you can see that the workshop is divided into sections.
Each section introduces a new concept or feature of the Rust language.\
To verify your understanding, each section is paired with an exercise that you need to solve.

We recommend you work on a branch, so you can easily track your progress and pull
in updates from the main repository, if needed:

```bash
cd rust-wasm-image-processing
git checkout -b my-solutions
```

All exercises are located in the `exercises` folder.
Each exercise is structured as a Rust module that contains a single `transform` function and instructions on what to do.

## Getting Started

Go through the [Dev Setup](#dev-setup) below and then **run `./serve.sh`** to open the book and start the backend. The serve script will automatically build your Rust code, and rerun it when you change things!

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
