#!/usr/bin/env bash
set -eux

rm -rf __tests__/**/__snapshots__
yarn run babel __tests__ -d tests/fixtures/