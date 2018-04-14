#[macro_use] extern crate honggfuzz;
extern crate fuzz_targets_common;
use fuzz_targets_common::*;

fn main() {
    loop {
        fuzz!(|data|{
            fuzz_regex(data);
        })
    }
}
