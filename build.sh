#!/bin/sh

# Build the .wasm module
cargo +nightly build --target wasm32-unknown-unknown

# Generate the rust/js bindings
wasm-bindgen ./target/wasm32-unknown-unknown/debug/wasm_webgl_ecs.wasm --out-dir .

# Format the generated js
prettier --write wasm_webgl_ecs.js

## Start the dev server
npm run serve
