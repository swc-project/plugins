#!/usr/bin/env bash
set -eu

rm -rf __tests__/**/__snapshots__

find __tests__ -type d -name '__fixtures__' -exec ./scripts/tests/process-dir.sh {} \;
