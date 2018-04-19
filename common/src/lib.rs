extern crate tendril;

extern crate brotli;
extern crate chrono;
extern crate crypto_hashes;
extern crate cssparser;
extern crate deflate;
extern crate dns_parser;
extern crate flac;
extern crate html5ever;
extern crate httparse;
extern crate humantime;
extern crate iso8601;
extern crate proc_macro2;
extern crate regex;
extern crate url;
extern crate bson;

// many function bodies are copied from https://github.com/rust-fuzz/targets

#[inline(always)]
pub fn fuzz_brotli(data: &[u8]) {
    use std::io::{Cursor, Read};

    let mut data_reader = Cursor::new(data);
    let mut result = Vec::with_capacity(data.len());

    let mut de = brotli::Decompressor::new(&mut data_reader, data.len());

    let _ = de.read_exact(&mut result);
}

#[inline(always)]
pub fn fuzz_bson(data: &[u8]) {
    let _ = bson::decode_document(&mut std::io::Cursor::new(data));
}

#[inline(always)]
pub fn fuzz_chrono(data: &[u8]) {
    use chrono::prelude::*;
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = DateTime::parse_from_rfc2822(data);
        let _ = DateTime::parse_from_rfc3339(data);
    }
}

#[inline(always)]
pub fn fuzz_crypto_hashes_blake2b(data: &[u8]) {
    use crypto_hashes::digest::Digest;
    let mut hasher = crypto_hashes::blake2::Blake2b::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_blake2s(data: &[u8]) {
    use crypto_hashes::digest::Digest;
    let mut hasher = crypto_hashes::blake2::Blake2s::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_gost94(data: &[u8]) {
    use crypto_hashes::digest::Digest;
    let mut hasher = crypto_hashes::gost94::Gost94Test::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_md2(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::md2::Md2::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_md4(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::md4::Md4::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_md5(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::md5::Md5::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_ripemd160(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::ripemd160::Ripemd160::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha1(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha1::Sha1::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha2_256(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha2::Sha256::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha2_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha2::Sha512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha3::Sha3_512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_keccak512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha3::Keccak512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_shake256(data: &[u8]) {
    use crypto_hashes::digest::{Input, ExtendableOutput};

    let mut hasher = crypto_hashes::sha3::Shake256::default();
    hasher.process(data);
    hasher.xof_result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_streebog_256(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::streebog::Streebog256::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_streebog_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::streebog::Streebog512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_whirlpool(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::whirlpool::Whirlpool::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_css_parser_read(data: &[u8]) {
    use cssparser::{Parser, ParserInput};

    if let Ok(str_) = std::str::from_utf8(data) {
        let mut parser_input = ParserInput::new(str_);
        let mut parser = Parser::new(&mut parser_input);
        while parser.next_including_whitespace_and_comments().is_ok() { }
    }
}

#[inline(always)]
pub fn fuzz_css_parser_read_write_read(data: &[u8]) {
    use cssparser::{Parser, ParserInput, ToCss, Token};

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
}

#[inline(always)]
pub fn fuzz_deflate(data: &[u8]) {
    let _compressed = deflate::deflate_bytes(&data);
}

#[inline(always)]
pub fn fuzz_dns_parser(data: &[u8]) {
    let _ = dns_parser::Packet::parse(data);
}

#[inline(always)]
pub fn fuzz_flac(data: &[u8]) {
    use flac::{ByteStream, Stream};

    let s = Stream::<ByteStream>::from_buffer(data);
    if let Ok(mut stream) = s {
        let _ = stream.info();
        let _ = stream.metadata();
        let mut iter = stream.iter::<i8>();
        while iter.next().is_some() { }
    }

}

#[inline(always)]
pub fn fuzz_html5ever(data: &[u8]) {
    use std::default::Default;
    use std::io::BufReader;

    use html5ever::driver::ParseOpts;
    use html5ever::tree_builder::TreeBuilderOpts;
    use html5ever::{parse_document, serialize};
    use html5ever::tendril::TendrilSink;
    use html5ever::rcdom::RcDom;

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
}

#[inline(always)]
pub fn fuzz_httparse_request(data: &[u8]) {
	let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let _ = req.parse(data);
}

#[inline(always)]
pub fn fuzz_httparse_response(data: &[u8]) {
	let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut res = httparse::Response::new(&mut headers);
    let _ = res.parse(data);
}

#[inline(always)]
pub fn fuzz_humantime(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = humantime::parse_duration(data);
    }
}

#[inline(always)]
pub fn fuzz_iso8601(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = iso8601::date(data);
        let _ = iso8601::time(data);
        let _ = iso8601::datetime(data);
    }
}

#[inline(always)]
pub fn fuzz_proc_macro2(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        if let Ok(token_stream) = data.parse::<proc_macro2::TokenStream>() {
            for _ in token_stream { }
        }
    }
}

#[inline(always)]
pub fn fuzz_regex(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        // split data into regular expression and actual input to search through
        use std::cmp::max;
        let len = data.chars().count();
        let split_off_point = max(len / 5, 1) as usize;
        let char_index = data.char_indices().nth(split_off_point);

        if let Some((char_index, _)) = char_index {
            let (pattern, input) = data.split_at(char_index);
            if let Ok(re) = regex::Regex::new(pattern) {
                re.is_match(input);
            }
        }
    }
}

#[inline(always)]
pub fn fuzz_url(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = url::Url::parse(s);
    }
}
