#!/usr/bin/env bash
set -eu


CRATES="$(cargo metadata --format-version 1 \
    | jq -r '.packages[] | select(.source == null) | .name')"


for CRATE in $CRATES
do
   cargo build --release -p $CRATE --target wasm32-wasi
done