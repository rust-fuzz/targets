#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate zip;

fuzz_target!(|data| {
    let reader = std::io::Cursor::new(data);
    let mut archive = if let Ok(x) = zip::ZipArchive::new(reader) { x } else { return; };

    for i in 0..archive.len() {
        use std::io::prelude::*;

        let file = archive.by_index(i).unwrap();
        let _size = file.bytes().count();
    }
});
