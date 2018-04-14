#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate fuzz_targets_common;
use fuzz_targets_common::*;

fuzz_target!(|data|{
    fuzz_iso8601(data);
});
