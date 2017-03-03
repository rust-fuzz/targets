#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate serde_json;

fuzz_target!(|data| {
    let _ = serde_json::from_slice::<serde_json::Value>(data);
});
