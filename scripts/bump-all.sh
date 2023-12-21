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



for CRATE in $CRATES
do
   bumpCargo $CRATE
   git commit -a -m "Bump cargo crate: ${CRATE}" || true
done

for PKG in ./packages/*; do
    bumpNpm $PKG
    git commit -a -m "Bump npm package: ${PKG}" || true
done
