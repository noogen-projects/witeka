#!/bin/sh

mode=${1:+"--release"}
out_dir=${1:-debug}

cargo build --target wasm32-unknown-unknown -p witeka-wasm-client $mode
wasm-bindgen --target web --no-typescript --out-dir static/app --out-name wasm_client target/wasm32-unknown-unknown/${out_dir}/witeka-wasm-client.wasm