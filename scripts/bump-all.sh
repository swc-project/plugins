#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd ./packages/$1 && yarn version minor)
    git tag -d $(git tag -l)
}

# Delete tags
git tag -d $(git tag -l)

bumpNpm emotion
bumpNpm jest
bumpNpm styled-components
bumpNpm styled-jsx
bumpNpm transform-imports
bumpNpm relay

# Delete tags
git tag -d $(git tag -l)
