#!/bin/sh

command -v rustup >/dev/null 2>&1 || {
  echo "'rustup' appears to be missing. Please refer to the installation instructions in README.md." >&2
  exit 1
}

if ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  echo "Rust target 'wasm32-unknown-unknown' appears to be missing. Please refer to the installation instructions in README.md." >&2
  exit 1
fi

command -v mdbook >/dev/null 2>&1 || {
  echo "'mdbook' appears to be missing. Please refer to the installation instructions in README.md." >&2
  exit 1
}

command -v wasm-pack >/dev/null 2>&1 || wasm-pack --version | awk '{split($2,v,"."); exit !(v[1]>0 || v[2]>=13)}' || {
  echo "'wasm-pack' >= 0.13 appears to be missing. Please refer to the installation instructions in README.md." >&2
  exit 1
}

command -v watchexec >/dev/null 2>&1 || {
  echo "'watchexec' appears to be missing. Please refer to the installation instructions in README.md." >&2
  exit 1
}

# We must use 127.0.0.1 as hostname for this to work in a dev container.
watchexec --watch transformers --watch frontend/src -r 'wasm-pack build frontend --target no-modules --out-dir wasm --no-typescript --no-pack --dev && mdbook serve --hostname 127.0.0.1 frontend' &
watchexec --watch transformers -r cargo run --bin backend

wait
