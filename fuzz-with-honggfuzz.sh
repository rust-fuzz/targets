#!/bin/sh -e

if [ "$#" -ne 1 ]; then
    echo "please give target name as argument" 1>&2
    exit 1
fi

./gen-targets-src.sh

cd fuzzer-honggfuzz

if [ -d "../common/seeds/$1" ]; then
	export HFUZZ_RUN_ARGS="-f ../common/seeds/$1 --covdir_all hfuzz_workspace/$1/input $HFUZZ_RUN_ARGS"
fi

cargo hfuzz run $1
