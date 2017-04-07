#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate crypto_hashes;
extern crate generic_array;

use crypto_hashes::digest::{Input, VariableOutput};
use generic_array::typenum::U256;

fuzz_target!(|data| {
    let mut buffer = [0; 256];
    let mut hasher = crypto_hashes::sha3::Shake256::default();
    hasher.digest(data);
    hasher.variable_result(&mut buffer);
});

