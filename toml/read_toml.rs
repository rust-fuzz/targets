#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate toml;

fuzz_target!(|data| {
    if let Ok(data) = toml::from_slice::<toml::Value>(data) {
        let _ = toml::to_string(&data);
    }
});
