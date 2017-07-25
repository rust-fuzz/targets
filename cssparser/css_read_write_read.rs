#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate cssparser;

use cssparser::{Parser, ParserInput, ToCss, Token};

fuzz_target!(|data: &[u8]| {
    // parse `data` into tokens
    let str1 = match std::str::from_utf8(data) {
        Ok(d) => d,
        Err(..) => return,
    };
    let tokens1: Vec<Token> = {
        let mut parser_input = ParserInput::new(str1);
        let mut parser = Parser::new(&mut parser_input);
        let mut tokens = vec![];
        while let Ok(token) = parser.next_including_whitespace_and_comments() {
            tokens.push(token.clone())
        }
        tokens
    };

    // dump the tokens into a string and parse again into tokens
    let str2 = tokens1.iter().map(|t| t.to_css_string()).collect::<String>();
    let tokens2: Vec<Token> = {
        let mut parser_input = ParserInput::new(&str2);
        let mut parser = Parser::new(&mut parser_input);
        let mut tokens = vec![];
        while let Ok(token) = parser.next_including_whitespace_and_comments() {
            tokens.push(token.clone())
        }
        tokens
    };

    assert_eq!(tokens1, tokens2);
});
