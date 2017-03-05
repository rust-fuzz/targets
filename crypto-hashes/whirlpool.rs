#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate whirlpool;

use whirlpool::Digest;

fuzz_target!(|data| {
    let mut hasher = whirlpool::Whirlpool::new();
    hasher.input(data);
    hasher.result();
});

