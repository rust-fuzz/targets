#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate sha3;

use sha3::Digest;

fuzz_target!(|data| {
    let mut hasher = sha3::Sha3_512::new();
    hasher.input(data);
    hasher.result();
});

