#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate mp4parse;

use std::io::Cursor;

fuzz_target!(|data| {
    let mut reader = Cursor::new(data);

    let mut context = mp4parse::MediaContext::new();
    let _ = mp4parse::read_mp4(&mut reader, &mut context);
});
