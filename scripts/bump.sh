#!/usr/bin/env bash
set -eu

./scripts/bump-npm.sh

cargo mono bump -i $@
git commit -a -m "Bump crates" || true
