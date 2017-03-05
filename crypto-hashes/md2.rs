#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate md2;

use md2::Digest;

fuzz_target!(|data| {
    let mut hasher = md2::Md2::new();
    hasher.input(data);
    hasher.result();
});

