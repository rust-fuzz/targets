#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate fuzz_targets_common;
use fuzz_targets_common::fuzz_###TARGET### as fuzz_target;

fuzz_target!(|data|{
    fuzz_target(data);
});
