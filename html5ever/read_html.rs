// adapted from https://github.com/servo/html5ever/blob/00c3c41f77cf6fbf325140906c88e4153dd19020/examples/html2html.rs

#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate tendril;
extern crate html5ever;

use std::default::Default;
use std::io::BufReader;

use html5ever::driver::ParseOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::rcdom::RcDom;

fuzz_target!(|data| {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut BufReader::new(data));

    let dom = if let Ok(dom) = dom { dom } else { return; };

    let mut out = Vec::with_capacity(data.len());
    let _ = serialize(&mut out, &dom.document, Default::default());
});
