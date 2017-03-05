#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate groestl;

use groestl::Digest;

fuzz_target!(|data| {
    let mut hasher = groestl::Groestl256::new();
    hasher.input(data);
    hasher.result();
});

