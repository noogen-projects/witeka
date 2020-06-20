#!/bin/sh

mode=${1:+"--release"}
flags=${1:+"-Clto -Copt-level=s"}
out_dir=${1:-debug}

RUSTFLAGS=$flags cargo build --target wasm32-unknown-unknown -p witeka-wasm-client $mode
wasm-bindgen --target web --no-typescript --out-dir static/app --out-name wasm_client target/wasm32-unknown-unknown/${out_dir}/witeka-wasm-client.wasm