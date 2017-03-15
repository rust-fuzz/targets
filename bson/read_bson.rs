#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate bson;

fuzz_target!(|data| {
    let _ = bson::decode_document(&mut std::io::Cursor::new(data));
});
