#!/bin/sh

trunk serve --config frontend --watch ../transformers --open --proxy-backend http://localhost:3000/api &
watchexec --watch transformers -r cargo run --bin backend
wait
