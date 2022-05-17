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
    git tag -d $(git tag -l)
}

# Delete tags
git tag -d $(git tag -l)

up swc_atoms
up swc_common
up testing
up swc_ecmascript
up swc_ecma_transforms_testing
up swc_plugin

up styled_jsx
up styled_components
up modularize_imports
up swc_emotion

bump swc_plugin_jest
bump swc_plugin_styled_jsx
bump swc_plugin_transform_imports
bump swc_plugin_styled_components

bumpNpm jest
bumpNpm styled-components
bumpNpm styled-jsx
bumpNpm transform-imports
bumpNpm relay

# Delete tags
git tag -d $(git tag -l)

# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null