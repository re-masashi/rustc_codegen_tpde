#!/bin/sh

export REAL_LIBRARY_PATH_VAR="LD_LIBRARY_PATH"
export LLVM_LINK_SHARED="true"
LLVM_CONFIG=$(which llvm-config)
export LLVM_CONFIG

set -e
echo "[BUILD] build system" 1>&2
exec cargo run --manifest-path "$(git rev-parse --show-toplevel)/build_system/Cargo.toml" -- "$@"
