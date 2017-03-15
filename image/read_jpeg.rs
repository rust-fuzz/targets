#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate jpeg_decoder;

fuzz_target!(|data| {
    let mut decoder = jpeg_decoder::Decoder::new(data);
    let _pixels = decoder.decode();
    let _metadata = decoder.info();
});
