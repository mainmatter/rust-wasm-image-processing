#!/bin/sh

watchexec --watch transformers --watch frontend/src -r 'wasm-pack build frontend --target no-modules --out-dir wasm --no-typescript && mdbook serve frontend' &
watchexec --watch transformers --watch frontend/src -r 'wasm-pack build frontend --target no-modules --out-dir wasm --no-typescript --dev && mdbook serve frontend' &
watchexec --watch transformers -r cargo run --bin backend

wait
