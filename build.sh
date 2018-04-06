#!/bin/sh

# Build the .wasm module
cargo +nightly build --release --target wasm32-unknown-unknown

# Generate the rust/js bindings
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_webgl_ecs.wasm --out-dir .

# Reduce size of .wasm output
wasm-gc ./wasm_webgl_ecs_bg.wasm ./wasm_webgl_ecs_bg.wasm

# TODO use wasm-opt to optimize output

# Format the generated js
prettier --write wasm_webgl_ecs.js

## Start the dev server
npm run serve
