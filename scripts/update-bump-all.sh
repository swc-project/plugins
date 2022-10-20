#!/usr/bin/env bash
set -eu

function up {
    cargo upgrade --workspace $@
}

up swc_atoms swc_common testing swc_ecmascript swc_ecma_transforms_testing swc_plugin swc_core

git commit -a -m 'Update swc crates' || true

up styled_jsx modularize_imports swc_emotion

git commit -a -m 'Update extra crates' || true

./scripts/bump-all.sh

# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null

git commit -a -m 'lockfile' || true
