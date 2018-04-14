#!/bin/sh -e

if [ "$#" -ne 1 ]; then
    echo "please give target name as argument" 1>&2
    exit 1
fi

cd fuzzer-afl
cargo afl build --release
mkdir -p "corpus-$1"
cargo afl fuzz -i "seed-corpus" -o "corpus-$1" -- "target/release/$1"
