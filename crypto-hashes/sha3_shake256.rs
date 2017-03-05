#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate sha3;
extern crate generic_array;

use sha3::Digest;
use generic_array::typenum::U256;

fuzz_target!(|data| {
    let mut hasher = sha3::Shake256::<U256>::new();
    hasher.input(data);
    hasher.result();
});

