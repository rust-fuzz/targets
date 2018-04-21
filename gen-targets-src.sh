#!/bin/sh -e

for D in fuzzer-*/; do
	mkdir -p "${D}/src/bin"
	rm -f ${D}/src/bin/*.rs
	cd common
	cargo run --bin gen-target-src "../targets.txt" "../${D}/template.rs" "../${D}/src/bin"
	cd ..
done
