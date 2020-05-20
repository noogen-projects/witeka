#!/bin/sh -e

remote="$1"

if [ "$remote" = "origin" ]; then
  cargo +nightly fmt -- --check
  cargo clippy --all-targets --workspace --all-features -- -D warnings
fi
