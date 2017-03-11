#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate blake2;

use blake2::Digest;

fuzz_target!(|data| {
    let mut hasher = blake2::Blake2b::new_keyed(&[]);
    hasher.input(data);
    hasher.result();
});

