#!/usr/bin/env bash
set -eu

cargo mono bump $@
git commit -a -m "Bump crates" || true
