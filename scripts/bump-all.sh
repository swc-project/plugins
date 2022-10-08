#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd $1 && yarn version --no-git-tag-version --patch)
    git tag -d $(git tag -l)
}

for PKG in ./packages/*; do
    bumpNpm $PKG
done

