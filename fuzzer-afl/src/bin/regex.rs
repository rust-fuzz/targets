extern crate afl;
extern crate fuzz_targets_common;
use fuzz_targets_common::*;

fn main() {
    afl::read_stdio_bytes(|data|{
        fuzz_regex(&data);
    });
}
