#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd $1 && yarn version --no-git-tag-version --patch)
}

function bumpCargo {
    cargo mono bump $1 --breaking
}

CRATES="$(cargo metadata --format-version 1 \
    | jq -r '.packages[] | select(.source == null) | .name')"


for PKG in ./packages/*; do
    bumpNpm $PKG
done

for CRATE in $CRATES
do
   bumpCargo $CRATE
done