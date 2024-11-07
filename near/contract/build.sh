#!/bin/sh

echo ">> Building contract"

set -e && RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/contract.wasm ./res/