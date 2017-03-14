#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate brotli;

fuzz_target!(|data| {
    use std::io::{Cursor, Read};

    let mut data_reader = Cursor::new(data);
    let mut result = Vec::with_capacity(data.len());

    let mut de = brotli::Decompressor::new(&mut data_reader, data.len());

    let _ = de.read_exact(&mut result);
});
