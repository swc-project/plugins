#!/usr/bin/env bash
set -eu

echo "Processing $1"
echo ""

TEST_DIR="${1/__tests__/}"  
TEST_DIR="${TEST_DIR/__fixtures__/}"  

yarn run babel $1 -d "tests/fixture/$TEST_DIR"
