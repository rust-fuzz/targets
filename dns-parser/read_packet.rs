#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate dns_parser;

fuzz_target!(|data| {
    let _ = dns_parser::Packet::parse(data);
});
