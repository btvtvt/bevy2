#!/bin/sh

export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

cargo build --release --target wasm32-unknown-unknown --bin client-wasm

wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "mygame" \
    target/wasm32-unknown-unknown/release/client-wasm.wasm

# ./target/wasm32-unknown-unknown/release/mygame.wasm
