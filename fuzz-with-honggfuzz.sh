#!/bin/sh -e

if [ "$#" -ne 1 ]; then
    echo "please give target name as argument" 1>&2
    exit 1
fi

cd fuzzer-honggfuzz
cargo hfuzz run $1
