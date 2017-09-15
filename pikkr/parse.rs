#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate pikkr;

fuzz_target!(|data| {
    let q = vec!["$.x".as_bytes()];
    let mut parser = pikkr::Pikkr::new(&q, 1).unwrap();
    let _ = parser.parse(data);
});
