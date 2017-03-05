#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate sha1;

use sha1::Digest;

fuzz_target!(|data| {
    let mut hasher = sha1::Sha1::new();
    hasher.input(data);
    hasher.result();
});

