#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate uuid;

use uuid::Uuid;

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = Uuid::parse_str(data);
    } else {
        let _ = Uuid::from_bytes(data);
    }
});
