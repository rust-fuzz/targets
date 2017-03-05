#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate streebog;

use streebog::Digest;

fuzz_target!(|data| {
    let mut hasher = streebog::Streebog256::new();
    hasher.input(data);
    hasher.result();
});

