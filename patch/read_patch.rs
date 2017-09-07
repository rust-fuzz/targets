#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate patch;

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = patch::parse(data);
    }
});
