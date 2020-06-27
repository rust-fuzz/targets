#[macro_use]
extern crate afl;
extern crate fuzz_targets_common;
use fuzz_targets_common::fuzz_###TARGET### as fuzz_target;

fn main() {
    afl::fuzz!(|data: &[u8]|{
        fuzz_target(&data);
    });
}
