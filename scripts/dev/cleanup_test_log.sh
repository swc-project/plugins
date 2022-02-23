#!/bin/sh

set -e

export NODE_PATH=$(npm root --quiet -g)
node ./scripts/dev/cleanup_test_log.js "$1" "$2"
diff -u ./clean-actual.js ./clean-expected.js > clean.diff
