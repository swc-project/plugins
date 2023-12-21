#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd $1 && yarn version --no-git-tag-version --patch)
}

cargo set-version --workspace --bump major

for PKG in ./packages/*; do
    bumpNpm $PKG
    git commit -a -m "Bump npm package: ${PKG}" || true
done
