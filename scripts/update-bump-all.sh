#!/usr/bin/env bash
set -eu

function up {
    cargo upgrade --workspace $@
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

up swc_atoms swc_common testing swc_ecmascript swc_ecma_transforms_testing swc_plugin

up styled_jsx styled_components modularize_imports swc_emotion

bump swc_plugin_jest
bump swc_plugin_styled_jsx
bump swc_plugin_transform_imports
bump swc_plugin_styled_components
bump swc_plugin_emotion

./scripts/bump-all.sh

# Delete tags
git tag -d $(git tag -l)

# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null