#!/usr/bin/env bash
set -eu

function up {
    cargo upgrade --workspace $@
}

up swc_atoms swc_common testing swc_ecmascript swc_ecma_transforms_testing swc_plugin swc_core

up styled_jsx modularize_imports swc_emotion

./scripts/bump-all.sh

# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null