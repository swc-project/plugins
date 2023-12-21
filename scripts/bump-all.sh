#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd $1 && yarn version --no-git-tag-version --patch)
}

cargo set-version --workspace --bump minor
git commit -a -m "Bump crates" || true

for PKG in ./packages/*; do
    bumpNpm $PKG
    git commit -a -m "Bump npm package: ${PKG}" || true
done
