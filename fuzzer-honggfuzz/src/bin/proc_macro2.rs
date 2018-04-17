#[macro_use] extern crate honggfuzz;
extern crate fuzz_targets_common;
use fuzz_targets_common::fuzz_url as fuzz_target;

fn main() {
    loop {
        fuzz!(|data|{
            fuzz_proc_macro2(data);
        })
    }
}
