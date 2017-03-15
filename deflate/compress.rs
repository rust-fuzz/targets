#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate deflate;

fuzz_target!(|data| {
    let _compressed = deflate::deflate_bytes(&data);
});
