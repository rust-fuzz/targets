#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate sha2;

use sha2::Digest;

fuzz_target!(|data| {
    let mut hasher = sha2::Sha256::new();
    hasher.input(data);
    hasher.result();
});

