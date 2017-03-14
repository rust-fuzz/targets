#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate chrono;

use chrono::prelude::*;

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = DateTime::parse_from_rfc2822(data);
        let _ = DateTime::parse_from_rfc3339(data);
    }
});
