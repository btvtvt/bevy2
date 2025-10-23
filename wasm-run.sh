#!/bin/sh

# export RUSTFLAGS='--cfg getrandom_backend="linux_getrandom"'

export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

# cargo run --target wasm32-unknown-unknown --bin client-wasm
cargo build --release --target wasm32-unknown-unknown --bin client-wasm
