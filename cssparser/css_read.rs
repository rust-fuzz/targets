#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate cssparser;

use cssparser::{Parser, ParserInput};

fuzz_target!(|data| {
    if let Ok(str_) = std::str::from_utf8(data) {
        let mut parser_input = ParserInput::new(str_);
        let mut parser = Parser::new(&mut parser_input);
        while parser.next_including_whitespace_and_comments().is_ok() { }
    }
});
