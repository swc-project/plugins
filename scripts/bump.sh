#!/usr/bin/env bash
set -eu

pnpm changeset || true
pnpm changeset version

for pkg in $(ls -d packages/*); do
    CHANGELOG=$(cat ./$pkg/CHANGELOG.md) envsubst < ./$pkg/README.md.tmpl > ./$pkg/README.md
    git add ./$pkg/README.md
done

git commit -am "Merge CHANGELOG into README"

cargo mono bump -i
git commit -a -m "Bump crates" || true
