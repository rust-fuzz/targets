#!/bin/sh -e

if [ "$#" -ne 1 ]; then
    echo "please give target name as argument" 1>&2
    exit 1
fi

cd fuzzer-libfuzzer

export RUSTFLAGS="$RUSTFLAGS \
--cfg fuzzing \
-C passes=sancov \
-C llvm-args=-sanitizer-coverage-level=4 \
-C llvm-args=-sanitizer-coverage-trace-pc-guard \
-C llvm-args=-sanitizer-coverage-prune-blocks=0 \
-C panic=abort \
-C debug-assertions=on \
-C debuginfo=0 \
-C opt-level=3 "

export ASAN_OPTIONS="$ASAN_OPTIONS detect_odr_violation=0"

if [ "`uname`" = "Darwin" ] ; then
    TARGET="x86_64-apple-darwin"
elif [ "`uname`" = "Linux" ] ; then
    TARGET="x86_64-unknown-linux-gnu"
    export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-sanitizer-coverage-trace-compares"
else
    echo "libfuzzer-sys only supports Linux and macOS" 1>&2
    exit 1
fi

mkdir -p "corpus-$1"
cargo run --target $TARGET --bin "$1" -- "corpus-$1" seed-corpus
