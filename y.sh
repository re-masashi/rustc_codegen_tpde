#!/bin/sh

set -e
echo "[BUILD] build system" 1>&2
exec cargo run --manifest-path "$(git rev-parse --show-toplevel)/build_system/Cargo.toml" -- "$@"
