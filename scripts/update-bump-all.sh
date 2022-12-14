#!/usr/bin/env bash
set -eu

function upgradeCargo {
    cargo upgrade -i $@ || true
}

upgradeCargo -p swc_atoms -p swc_common -p testing -p swc_ecmascript -p swc_ecma_transforms_testing -p swc_plugin -p swc_core

git commit -a -m 'Update swc crates' || true

upgradeCargo -p styled_jsx -p modularize_imports -p swc_emotion

git commit -a -m 'Update extra crates' || true

./scripts/bump-all.sh

# Ensure that Cargo.lock is up-to-date
cargo metadata --offline --format-version 1 > /dev/null

git commit -a -m 'lockfile' || true
