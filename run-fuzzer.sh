#!/bin/bash

set -e

# Specify RUSTFLAGS:
export RUSTFLAGS=""
# - so the target crate is compiled with sanitization
export RUSTFLAGS="$RUSTFLAGS -C passes=sancov -C llvm-args=-sanitizer-coverage-level=3 -Z sanitizer=address -C panic=abort"
# - optimizations
export RUSTFLAGS="$RUSTFLAGS -C opt-level=3"
# - and all debug infos
export RUSTFLAGS="$RUSTFLAGS -C debug-assertions=on -C debuginfo=2"

# Specify asan options to disable things that don't work
export ASAN_OPTIONS="$ASAN_OPTIONS detect_odr_violation=0"

# Show all the rust errors
export RUST_BACKTRACE=full

# If Travis CI executes this script, just build all the crates with the env vars set
if [ -n "$CI" ]; then
    cargo build -j 1 --all --verbose --target x86_64-unknown-linux-gnu
    exit 0
fi

if [ "$#" -lt 2 ]; then
    echo "Usage: run-fuzzer.sh <crate> <target> [<options...>]" 1>&2
    exit 1
fi

# Change directory to the crate we want to fuzz
cd "$1"

# Create seed directory if it does not exist. Add example files here.
mkdir -p seeds

# Create corpus directory which the fuzzer will fill with interesting inputs.
mkdir -p corpus

# Run the fuzzer with that target
if [ "$(uname -s)" == "Darwin" ]; then
    TARGET="x86_64-apple-darwin"
elif [ "$(uname -s)" == "Linux" ]; then
    TARGET="x86_64-unknown-linux-gnu"
else
    echo "Sorry, only Mac OS and Linux are supported"
    exit 1
fi

if [ "$3" == "minimize" ]; then
    OPTS="-minimize_crash=1 $4"
else
    OPTS="${@:3} $(pwd)/corpus $(pwd)/seeds"
fi

cargo run --target $TARGET --bin "$2" -- $OPTS

