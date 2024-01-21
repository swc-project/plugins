#!/usr/bin/env bash
set -eu

pnpm changeset
pnpm changeset version

cargo set-version --workspace --bump minor
git commit -a -m "Bump crates" || true
