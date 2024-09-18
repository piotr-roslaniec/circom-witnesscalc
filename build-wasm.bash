#!/usr/bin/env bash

set -eu

rm -rf pkg-web pkg-nodejs

# TODO: Do I still need that?
export LIBCLANG_PATH=/usr/lib/llvm-17/lib

# Corresponding cargo command:
#cargo build --features wasm --release --target=wasm32-unknown-unknown

# Dev builds are useful for debugging
#wasm-pack build --features=wasm --target=web --out-dir pkg-web --dev
#wasm-pack build --features=wasm --target=nodejs --out-dir pkg-nodejs --dev

wasm-pack build --features=wasm --target=nodejs --out-dir pkg-nodejs --release
