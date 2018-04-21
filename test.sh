#!/bin/sh -ve

./gen-targets-src.sh

cargo install afl honggfuzz --force

# AFL

cd fuzzer-afl
cargo afl build
cd ..

# Honggfuzz

cd fuzzer-honggfuzz
cargo hfuzz build-debug
cd ..

# LibFuzzer

cd fuzzer-libfuzzer

export RUSTFLAGS="$RUSTFLAGS \
--cfg fuzzing \
-C passes=sancov \
-C llvm-args=-sanitizer-coverage-level=4 \
-C llvm-args=-sanitizer-coverage-trace-pc-guard \
-C llvm-args=-sanitizer-coverage-prune-blocks=0 \
-C debug-assertions=on \
-C debuginfo=0 \
-C opt-level=3 "

if [ "`uname`" = "Darwin" ] ; then
    TARGET="x86_64-apple-darwin"
elif [ "`uname`" = "Linux" ] ; then
    TARGET="x86_64-unknown-linux-gnu"
    export RUSTFLAGS="$RUSTFLAGS -C llvm-args=-sanitizer-coverage-trace-compares"
else
    echo "libfuzzer-sys only supports Linux and macOS" 1>&2
    exit 1
fi

cargo build --target $TARGET
cd ..