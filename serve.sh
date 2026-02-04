#!/bin/sh

if ! command -v rustup >/dev/null 2>&1; then
  echo "'rustup' is required. Please refer to the setup instructions in README.md." >&2
  exit 1
fi

if ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  echo "Rust target 'wasm32-unknown-unknown' is required. Please refer to the setup instructions in README.md." >&2
  exit 1
fi

if ! command -v mdbook >/dev/null 2>&1; then
  echo "'mdbook' is required. Please refer to the setup instructions in README.md." >&2
  exit 1
fi

if ! command -v wasm-pack >/dev/null 2>&1 || ! wasm-pack --version | awk '{split($2,v,"."); exit !(v[1]>0 || v[2]>=13)}'; then
  echo "'wasm-pack' >= 0.13 is required. Please refer to the setup instructions in README.md." >&2
  exit 1
fi

if ! command -v watchexec >/dev/null 2>&1; then
  echo "'watchexec' is required. Please refer to the setup instructions in README.md." >&2
  exit 1
fi

# We must bind to 0.0.0.0 for this to work in a dev container.
watchexec --watch exercises --watch frontend/src -r 'wasm-pack build frontend --target no-modules --out-dir wasm --no-typescript --no-pack --dev && mdbook serve --hostname 0.0.0.0 -p 3000 frontend' &

watchexec --watch exercises --watch  backend/src -r 'cargo run --target-dir target-serve --bin backend'

wait
