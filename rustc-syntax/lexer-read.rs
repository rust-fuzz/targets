#![feature(rustc_private)]
#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate syntax;
extern crate syntax_pos;

use std::path::PathBuf;
use std::rc::Rc;
use syntax::codemap::{CodeMap, FilePathMapping};
use syntax::parse::{token, ParseSess};
use syntax::parse::lexer::StringReader;

fuzz_target!(|bytes: &[u8]| {
    let s = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(..) => return,
    };
    let cm = Rc::new(CodeMap::new(FilePathMapping::empty()));
    let sess = ParseSess::new(FilePathMapping::empty());
    let fm = cm.new_filemap(PathBuf::from("silvasean.rs").into(), s.to_owned());
    let mut lexer = StringReader::new(&sess, fm);
    while let Ok(next) = lexer.try_next_token() {
        if next.tok == token::Token::Eof {
            return;
        }
    }
});
