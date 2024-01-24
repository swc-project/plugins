#!/usr/bin/env bash
set -eu

pnpm changeset
pnpm changeset version

for pkg in $(ls -d packages/*); do
    CHANGELOG=$(cat ./$pkg/CHANGELOG.md) envsubst < ./$pkg/README.md.tmpl > ./$pkg/README.md
    git add ./$pkg/README.md
done

git commit -am "Merge CHANGELOG into README"

cargo mono bump -i
# cargo set-version --workspace --bump minor
git commit -a -m "Bump crates" || true
