#!/usr/bin/env bash
set -eu

function up {
    cargo upgrade --workspace $1
}

function bump {
    cargo mono bump $1 --breaking
}

up swc_common
up tesitng
up swc_ecmascript
up swc_ecma_transforms_testing
up swc_plugin


bump jest
bump styled_components

# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null