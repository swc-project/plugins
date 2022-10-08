#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd $1 && yarn version --no-git-tag-version --patch)
}

function bumpCargo {
    cargo mono bump $1 --breaking
}

for PKG in ./packages/*; do
    bumpNpm $PKG
done


bumpCargo swc_plugin_jest
bumpCargo swc_plugin_styled_jsx
bumpCargo swc_plugin_transform_imports
bumpCargo swc_plugin_styled_components
bumpCargo swc_plugin_emotion
