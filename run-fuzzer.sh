#!/bin/sh

set -ex

if [ "$#" -ne 2 ]; then
    echo "Usage: run-fuzzer.sh <crate> <target>" 1>&2
    exit 1
fi

# Specify RUSTFLAGS so the target crate is compiled with sanitization
export RUSTFLAGS="-Cpasses=sancov -Cllvm-args=-sanitizer-coverage-level=3 -Zsanitizer=address -Cpanic=abort"

# Change directory to the crate we want to fuzz
cd "$1"

# Run the fuzzer with that target
cargo run --bin "$2"
