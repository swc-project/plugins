#!/usr/bin/env bash
set -eu

function up {
    cargo upgrade --workspace $1
}

up swc_common
up swc_ecmascript
up swc_plugin