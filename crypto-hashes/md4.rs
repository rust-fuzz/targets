#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate md4;

use md4::Digest;

fuzz_target!(|data| {
    let mut hasher = md4::Md4::new();
    hasher.input(data);
    hasher.result();
});

