#!/usr/bin/env bash
set -eu

pnpm changeset
pnpm changeset version

for $pkg in $(ls -d packages/*); do
    CHANGELOG=$(cat ./packages/$pkg/CHANGELOG.md) envsubst < ./packages/$pkg/README.md.tmpl > ./packages/$pkg/README.md
    git add ./packages/$pkg/README.md
done

git commit -am "Merge CHANGELOG into README"

cargo set-version --workspace --bump minor
git commit -a -m "Bump crates" || true
