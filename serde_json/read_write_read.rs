#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate serde_json;

fuzz_target!(|bytes1: &[u8]| {
    let parsed1 = match serde_json::from_slice::<serde_json::Value>(bytes1) {
        Ok(p) => p,
        Err(..) => return,
    };
    let bytes2 = serde_json::to_vec(&parsed1).unwrap();
    let parsed2 = match serde_json::from_slice::<serde_json::Value>(&bytes2) {
        Ok(p) => p,
        Err(..) => return,
    };
    assert_eq!(parsed1, parsed2);
});
