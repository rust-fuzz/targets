#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate serde_yaml;

fuzz_target!(|data| {
    let value = match serde_yaml::from_slice::<serde_yaml::Value>(data) {
        Ok(v) => v,
        Err(_) => return,
    };
    let serialized = match serde_yaml::to_vec(&value) {
        Ok(s) => s,
        Err(_) => return,
    };
    if let Ok(v) = serde_yaml::from_slice::<serde_yaml::Value>(&serialized) {
        assert_eq!(v, value);
    }
});
