#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate ripemd160;

use ripemd160::Digest;

fuzz_target!(|data| {
    let mut hasher = ripemd160::Ripemd160::new();
    hasher.input(data);
    hasher.result();
});

