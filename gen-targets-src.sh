#!/bin/sh -e

for D in fuzzer-*/; do
	cd common
	cargo run --bin gen-target-src "../targets.txt" "../${D}/template.rs" "../${D}/src/bin"
	cd ..
done
