#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate gif;

fuzz_target!(|data| {
    let decoder = gif::Decoder::new(std::io::Cursor::new(data));

    if let Ok(mut decoder) = decoder.read_info() {
        while let Ok(Some(_frame)) = decoder.read_next_frame() { }
    }
});
