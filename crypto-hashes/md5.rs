#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate md5;

use md5::Digest;

fuzz_target!(|data| {
    let mut hasher = md5::Md5::new();
    hasher.input(data);
    hasher.result();
});

