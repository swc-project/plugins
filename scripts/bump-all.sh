#!/usr/bin/env bash
set -eu

function bumpNpm {
    (cd ./packages/$1 && yarn version --no-git-tag-version --patch)
    git tag -d $(git tag -l)
}

# Delete tags
git tag -d $(git tag -l)

bumpNpm emotion
bumpNpm jest
bumpNpm loadable-components
bumpNpm styled-components
bumpNpm styled-jsx
bumpNpm transform-imports
bumpNpm relay

# Delete tags
git tag -d $(git tag -l)
