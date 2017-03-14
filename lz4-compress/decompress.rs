#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate lz4_compress;

fuzz_target!(|data: &[u8]| {
    if let Ok(_) = lz4_compress::decompress(data) {}
});
