# Contentful Wasm Workshop

- setup 6 exercises
  - 01 no conditional logic => just call photon function (flip, mirror)
  - 02 basic control flow => switch between two photon functions, show off number types (args to functions)
  - 03 show off match => switch between filters a number of filters based on a string param (panic on unknown)
  
  - talk about wasm vs server vs edge => could also be interruption / break for participants and show this off live
  
  - 04 error handling => building off of 03, show off proper error handling in Rust
  - 05 vec/slices, for loop => resize image into multiple size points? based on slice of sizes
  - 06 closures/map/collect/par-iter??? => building off of 05, show off more "elegant" Rust and rayon (https://github.com/RReverser/wasm-bindgen-rayon)
  
- setup workshop harness
  - use trunk https://trunkrs.dev
  - configure trunk to build the wasm for "in-browser wasm env"
  - either configure trunk to restart "backend env" or use cargo watch
  - frontend
    - page per exercise
    - dropdown to select environment
    - wasm env => call wasm-bindgeb generated JS
    - server(/edge) => fetch against endpoint 
    - input element to specify source image URL (with a default selection)
  - environment crates
    - glue code for calling the core exercise functions
    - should handle image URL fetching
    - "backend"
    - "wasm"
    - ("edge")


TODOs @Dax
- setup repo structure
  - 6 blank crates / 6 modules for exercises
  - frontend folder with blank index.html
  - simple axum server that exposes exercises
  - make sure everything builds correctly and everything watches correctly
  
TODOs @Jonas
- flesh out exercises
- make sure exercises compile to wasm
- setup nix/devcontainers/setup instructions

TODOs @Marco
- setup frontend
  - URL input for image URL (global) could use pexels as default
  - dropdown to switch between envs
  - one page per exercise
    - image tag for rendering output
    - input elements mapping to transform inputs (TODO wait for jonas to clarify what inputs are needed)
    - exercise description
  - must be plain html/css/js unfortunately bc trunk
  - but can use tailwind (trunk supports this)
