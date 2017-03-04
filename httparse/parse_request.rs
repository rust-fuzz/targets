#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate httparse;

fuzz_target!(|data| {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let _ = req.parse(data);
});
