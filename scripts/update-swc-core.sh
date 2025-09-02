#!/usr/bin/env bash
set -eu

curl https://raw.githubusercontent.com/swc-project/swc/main/scripts/update-all-swc-crates.sh | bash -s

git add -A && git commit -m "build: Update swc core"

echo "Bumping crates"
./scripts/bump-crates.sh --breaking

echo "Bumping npm packages"
./scripts/bump-npm.sh