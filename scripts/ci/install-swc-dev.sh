#!/usr/bin/env bash
set -eu

LATEST_TAG=$(curl -s https://api.github.com/repos/swc-project/swc-dev/releases/latest | jq -r '.tag_name')

echo "Downloading swc-dev@${LATEST_TAG}"

BIN_URL="https://github.com/swc-project/swc-dev/releases/download/$LATEST_TAG/swc-dev-$RUNNER_OS"

echo "Using $BIN_URL"

mkdir -p swc-dev-built
curl $BIN_URL -o swc-dev-built/swc-dev
chmod +x swc-dev-built/swc-dev
echo "$(pwd)/swc-dev-built" >> $GITHUB_PATH

echo $LATEST_TAG