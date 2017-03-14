#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate zopfli;

fuzz_target!(|data| {
    let options = zopfli::Options::default();

    for output_type in &[
        zopfli::Format::Deflate,
        zopfli::Format::Gzip,
        zopfli::Format::Zlib,
    ] {
        let mut res = Vec::with_capacity(data.len() / 2);
        let _ = zopfli::compress(&options, &output_type, &data, &mut res);
    }
});
