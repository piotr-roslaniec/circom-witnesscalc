#!/usr/bin/env bash

set -eu

rm -rf pkg-web pkg-nodejs

# TODO: Do I need that?
export LIBCLANG_PATH=/usr/lib/llvm-17/lib

# Corresponding cargo command:
#cargo build --features wasm --release --target=wasm32-unknown-unknown

#wasm-pack build --features=wasm --target=web --out-dir pkg-web --dev
wasm-pack build --features=wasm --target=nodejs --out-dir pkg-nodejs --dev