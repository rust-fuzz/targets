#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate httparse;

fuzz_target!(|data| {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut res = httparse::Response::new(&mut headers);
    let _ = res.parse(data);
});
