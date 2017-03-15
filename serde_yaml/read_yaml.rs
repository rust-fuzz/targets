#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate serde_yaml;

fuzz_target!(|data| {
    let _ = serde_yaml::from_slice::<serde_yaml::Value>(data);
});
