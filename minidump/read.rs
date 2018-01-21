#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate minidump;

use minidump::Minidump;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let cursor = Cursor::new(data);
    let _ = minidump::Minidump::read(cursor);
});
