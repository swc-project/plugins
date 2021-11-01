#!/usr/bin/env bash
set -eu

LATEST_TAG=$(curl -s https://api.github.com/repos/swc-project/swc-dev/releases/latest | jq -r '.tag_name')

echo "Downloading swc-dev@${LATEST_TAG}"

BIN_URL="https://github.com/swc-project/swc-dev/releases/download/$LATEST_TAG/swc-dev-$RUNNER_OS"

echo "Using `$BIN_URL`"
echo $LATEST_TAG