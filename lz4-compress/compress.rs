#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate lz4_compress;

fuzz_target!(|data: &[u8]| {
    lz4_compress::compress(data);
});
