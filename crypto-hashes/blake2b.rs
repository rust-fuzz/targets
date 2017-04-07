#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate crypto_hashes;

use crypto_hashes::digest::Digest;

fuzz_target!(|data| {
    let mut hasher = crypto_hashes::blake2::Blake2b::default();
    hasher.input(data);
    hasher.result();
});

