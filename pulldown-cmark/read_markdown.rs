#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate pulldown_cmark;

fuzz_target!(|data| {
    if let Ok(s) = std::str::from_utf8(data) {
        let parser = pulldown_cmark::Parser::new(s);
        for _ in parser { }
    }
});

