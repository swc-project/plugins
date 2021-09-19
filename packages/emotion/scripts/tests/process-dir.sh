#!/usr/bin/env bash
set -eu

echo "Processing $1"
echo ""

TEST_DIR="${1/__tests__/}"  
TEST_DIR="${TEST_DIR/__fixtures__/}"  


for filename in $1/*.js; do
    TEST_NAME="$(basename $filename .js)"
    echo "$filename => $TEST_DIR$TEST_NAME"

    yarn run babel $filename -o "tests/fixtures/$TEST_DIR$TEST_NAME/output.js"
    cp $filename "tests/fixtures/$TEST_DIR$TEST_NAME/input.js"
done