#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate gost94;

use gost94::Digest;

fuzz_target!(|data| {
    let mut hasher = gost94::Gost94Test::new();
    hasher.input(data);
    hasher.result();
});

