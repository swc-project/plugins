#!/usr/bin/env bash
set -eu

function up {
    cargo upgrade --workspace $1
}

function bump {
    cargo mono bump $1 --breaking
}

function bumpNpm {
    (cd ./packages/$1 && yarn version)
}

up swc_atoms
up swc_common
up testing
up swc_ecmascript
up swc_ecma_transforms_testing
up swc_plugin

bump swc_plugin_jest
bump styled_components
bump swc_plugin_styled_jsx

bumpNpm jest
bumpNpm styled-components
bumpNpm styled-jsx


# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null