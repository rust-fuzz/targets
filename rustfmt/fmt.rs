#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate rustfmt_nightly;

use rustfmt_nightly::{format_input, Input};
use rustfmt_nightly::config::Config;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut out = Vec::with_capacity(2_048);

        if let Ok(_) = format_input(
            Input::Text(s.to_string()),
            &Config::default(),
            Some(&mut out),
        ) {}
    }
});
