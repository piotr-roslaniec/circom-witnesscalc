#!/usr/bin/env bash

clear

export LIBCLANG_PATH=/usr/lib/llvm-17/lib

#cargo build --features wasm --release --target=wasm32-unknown-unknown

wasm-pack build --features=wasm --target=web