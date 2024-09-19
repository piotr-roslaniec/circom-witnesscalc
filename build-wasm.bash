#!/usr/bin/env bash

set -eu

rm -rf pkg-web pkg-nodejs

# TODO: Do I still need that?
export LIBCLANG_PATH=/usr/lib/llvm-17/lib

# Corresponding cargo command:
#cargo build --features wasm --release --target=wasm32-unknown-unknown

# Dev builds are useful for debugging, they also allow us to view logs
#wasm-pack build --features=wasm --target=web --out-dir pkg-web --dev
#wasm-pack build --features=wasm --target=nodejs --out-dir pkg-nodejs --dev

wasm-pack build --features=wasm --target=web --out-dir pkg-web --release
wasm-pack build --features=wasm --target=nodejs --out-dir pkg-nodejs --release

#export RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals"
#rustup run nightly-2022-12-12 wasm-pack build --target web --out-dir web-parallel -- --features parallel -Z build-std=panic_abort,std
