extern crate afl;
extern crate fuzz_targets_common;
use fuzz_targets_common::fuzz_###TARGET### as fuzz_target;

fn main() {
    afl::read_stdio_bytes(|data|{
        fuzz_target(&data);
    });
}
