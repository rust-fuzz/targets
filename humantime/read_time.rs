#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate humantime;

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = humantime::parse_duration(data);
    }
});
