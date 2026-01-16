# Contentful Wasm Workshop

- setup 6 exercises
  - 01 no conditional logic => just call photon function (flip, mirror)
  - 02 show off control flow & match => switch between filters a number of filters based on a string param (panic on unknown)
  
  - talk about wasm vs server vs edge => could also be interruption / break for participants and show this off live
  
  - 03 vec/slices, for loop => resize image into multiple size points? based on slice of sizes
  - 04 closures/map/collect/par-iter??? => building off of 05, show off more "elegant" Rust and rayon (https://github.com/RReverser/wasm-bindgen-rayon)

TODOs @Jonas
- flesh out exercises
- setup devcontainers/setup instructions

TODOs @Marco

# Dev dependencies

- Rust should be installed using [rustup](https://rustup.rs/)
- After that add the Wasm target: `rustup target add wasm32-unknown-unknown`
- Install the following tools:
  - trunk (e.g. through `cargo install --locked trunk`)
  - watchexec (e.g. through `cargo install --locked watchexec-cli`)
