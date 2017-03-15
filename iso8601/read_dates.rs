#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate iso8601;

fuzz_target!(|data| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = iso8601::date(data);
        let _ = iso8601::time(data);
        let _ = iso8601::datetime(data);
    }
});
