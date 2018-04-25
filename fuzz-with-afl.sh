#!/bin/sh -e

if [ "$#" -ne 1 ]; then
    echo "please give target name as argument" 1>&2
    exit 1
fi

./gen-targets-src.sh

cd fuzzer-afl

if [ -d "../common/seeds/$1" ]; then
	SEEDS="../common/seeds/$1"
else
	SEEDS="../common/seeds/nullbyte"
fi

cargo afl build --release
mkdir -p "corpus-$1"
cargo afl fuzz -i $SEEDS -o "corpus-$1" -- "target/release/$1"
