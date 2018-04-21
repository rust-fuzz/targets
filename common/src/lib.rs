extern crate tendril;
extern crate openssl;

extern crate brotli;
extern crate bson;
extern crate chrono;
extern crate crypto_hashes;
extern crate cssparser;
extern crate deflate;
extern crate dns_parser;
extern crate flac;
extern crate gif;
extern crate html5ever;
extern crate httparse;
extern crate humantime;
extern crate image;
extern crate iso8601;
extern crate jpeg_decoder;
extern crate minidump;
extern crate mp4parse;
extern crate patch;
extern crate pikkr;
extern crate png;
extern crate proc_macro2;
extern crate pulldown_cmark;
extern crate quick_xml;
extern crate regex;
extern crate ring;
extern crate semver;
extern crate serde_json;
extern crate serde_yaml;
extern crate tar;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate xml;
extern crate zip;
extern crate zopfli;

// many function bodies are copied from https://github.com/rust-fuzz/targets

#[inline(always)]
pub fn fuzz_brotli_read(data: &[u8]) {
    use std::io::{Cursor, Read};

    let mut data_reader = Cursor::new(data);
    let mut result = Vec::with_capacity(data.len());

    let mut de = brotli::Decompressor::new(&mut data_reader, data.len());

    let _ = de.read_exact(&mut result);
}

#[inline(always)]
pub fn fuzz_bson_read(data: &[u8]) {
    let _ = bson::decode_document(&mut std::io::Cursor::new(data));
}

#[inline(always)]
pub fn fuzz_chrono_read(data: &[u8]) {
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
pub fn fuzz_crypto_hashes_streebog_256(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::streebog::Streebog256::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_streebog_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::streebog::Streebog512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_whirlpool(data: &[u8]) {
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
pub fn fuzz_deflate_compress(data: &[u8]) {
    let _compressed = deflate::deflate_bytes(&data);
}

#[inline(always)]
pub fn fuzz_dns_parser_read_paquet(data: &[u8]) {
    let _ = dns_parser::Packet::parse(data);
}

#[inline(always)]
pub fn fuzz_flac_read(data: &[u8]) {
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
pub fn fuzz_gif_read(data: &[u8]) {
    let decoder = gif::Decoder::new(std::io::Cursor::new(data));

    if let Ok(mut decoder) = decoder.read_info() {
        while let Ok(Some(_frame)) = decoder.read_next_frame() { }
    }
}

#[inline(always)]
pub fn fuzz_html5ever_read(data: &[u8]) {
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
pub fn fuzz_humantime_read(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = humantime::parse_duration(data);
    }
}

#[inline(always)]
pub fn fuzz_image_read(data: &[u8]) {
    let _ = image::load_from_memory(data);
}

#[inline(always)]
pub fn fuzz_iso8601_read(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = iso8601::date(data);
        let _ = iso8601::time(data);
        let _ = iso8601::datetime(data);
    }
}

#[inline(always)]
pub fn fuzz_jpeg_decoder_read(data: &[u8]) {
    let mut decoder = jpeg_decoder::Decoder::new(data);
    let _pixels = decoder.decode();
    let _metadata = decoder.info();
}

#[inline(always)]
pub fn fuzz_minidump_read(data: &[u8]) {
    use std::io::Cursor;

    let cursor = Cursor::new(data);
    let _ = minidump::Minidump::read(cursor);
}

#[inline(always)]
pub fn fuzz_mp4parse_read(data: &[u8]) {
    use std::io::Cursor;

    let mut reader = Cursor::new(data);

    let mut context = mp4parse::MediaContext::new();
    let _ = mp4parse::read_mp4(&mut reader, &mut context);
}

#[inline(always)]
pub fn fuzz_patch_read(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = patch::parse(data);
    }
}

#[inline(always)]
pub fn fuzz_pikkr_read(data: &[u8]) {
    let q = vec!["$.x".as_bytes()];
    let mut parser = pikkr::Pikkr::new(&q, 1).unwrap();
    let _ = parser.parse(data);
}

fn png_decode(data: &[u8]) -> Result<(png::OutputInfo, Vec<u8>), ()> {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = decoder.read_info().map_err(|_| ())?;

    if info.buffer_size() > 50_000_000 {
        return Err(());
    }

    let mut img_data = Vec::with_capacity(info.buffer_size());
    reader.next_frame(&mut img_data).map_err(|_| ())?;

    Ok((info, img_data))
}

fn png_encode(info: &png::OutputInfo, data: &[u8]) -> Result<Vec<u8>, ()> {
    use png::HasParameters;

    let mut out = Vec::with_capacity(data.len());

    {
        let mut encoder = png::Encoder::new(&mut out, info.width, info.height);
        encoder.set(info.color_type).set(info.bit_depth);
        let mut writer = encoder.write_header().map_err(|_| ())?;
        writer.write_image_data(&data).map_err(|_| ())?;
    }

    Ok(out)
}

#[inline(always)]
pub fn fuzz_png_read(data: &[u8]) {
    if let Ok((info, pixels)) = png_decode(data) {
        let encoded = png_encode(&info, &pixels).expect("encode fail");
        let (_info2, pixels2) = png_decode(&encoded).expect("decode failed");

        assert_eq!(pixels, pixels2);
    }
}

#[inline(always)]
pub fn fuzz_png_read_write_read(data: &[u8]) {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = match decoder.read_info() {
        Ok(r) => r,
        Err(_) => return
    };

    if info.buffer_size() > 50_000_000 {
        return;
    }

    let mut img_data = vec![0; info.buffer_size()];
    let _ = reader.next_frame(&mut img_data);
}

#[inline(always)]
pub fn fuzz_proc_macro2_read(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        if let Ok(token_stream) = data.parse::<proc_macro2::TokenStream>() {
            for _ in token_stream { }
        }
    }
}

#[inline(always)]
pub fn fuzz_pulldown_cmark_read(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        let parser = pulldown_cmark::Parser::new(s);
        for _ in parser { }
    }
}

#[inline(always)]
pub fn fuzz_quick_xml_read(data: &[u8]) {
    use quick_xml::Reader;
    use std::io::Cursor;

    let cursor = Cursor::new(data);
    let mut reader = Reader::from_reader(cursor);
    let mut buf = vec![];
    loop {
        match reader.read_event(&mut buf) {
            Ok(quick_xml::events::Event::Eof) | Err(..) => break,
            _ => buf.clear(),
        }
    }
}

#[inline(always)]
pub fn fuzz_regex_is_match(data: &[u8]) {
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
pub fn fuzz_ring_digest_sha1(data: &[u8]) {
    assert_eq!(
        ring::digest::digest(
            &ring::digest::SHA1,
            data
        ).as_ref(),
        &*openssl::hash::hash2(
            openssl::hash::MessageDigest::sha1(),
            data
        ).unwrap()
    )
}

#[inline(always)]
pub fn fuzz_ring_digest_sha256(data: &[u8]) {
    assert_eq!(
        ring::digest::digest(
            &ring::digest::SHA256,
            data
        ).as_ref(),
        &*openssl::hash::hash2(
            openssl::hash::MessageDigest::sha256(),
            data
        ).unwrap()
    )
}

#[inline(always)]
pub fn fuzz_ring_digest_sha384(data: &[u8]) {
    assert_eq!(
        ring::digest::digest(
            &ring::digest::SHA384,
            data
        ).as_ref(),
        &*openssl::hash::hash2(
            openssl::hash::MessageDigest::sha384(),
            data
        ).unwrap()
    )
}

#[inline(always)]
pub fn fuzz_ring_digest_sha512(data: &[u8]) {
    assert_eq!(
        ring::digest::digest(
            &ring::digest::SHA512,
            data
        ).as_ref(),
        &*openssl::hash::hash2(
            openssl::hash::MessageDigest::sha512(),
            data
        ).unwrap()
    )
}

#[inline(always)]
pub fn fuzz_semver_read_write_read(data: &[u8]) {
    let data = match ::std::str::from_utf8(data) {
        Ok(d) => d,
        Err(..) => return,
    };
    let version = match semver::Version::parse(data) {
        Ok(v) => v,
        Err(..) => return,
    };
    let version_s = version.to_string();
    assert_eq!(version, semver::Version::parse(&version_s).unwrap());
}

#[inline(always)]
pub fn fuzz_semver_req_read_write_read(data: &[u8]) {
    let data = match ::std::str::from_utf8(data) {
        Ok(d) => d,
        Err(..) => return,
    };
    let version_req = match semver::VersionReq::parse(data) {
        Ok(v) => v,
        Err(..) => return,
    };
    let version_req_s = version_req.to_string();
    assert_eq!(version_req, semver::VersionReq::parse(&version_req_s).unwrap());
}

#[inline(always)]
pub fn fuzz_serde_json_read(data: &[u8]) {
    let _ = serde_json::from_slice::<serde_json::Value>(data);
}

#[inline(always)]
pub fn fuzz_serde_json_read_write_read(data: &[u8]) {
    let value = match serde_json::from_slice::<serde_json::Value>(data) {
        Ok(v) => v,
        Err(..) => return,
    };
    let serialized = serde_json::to_vec(&value).unwrap();
    let value2 = match serde_json::from_slice::<serde_json::Value>(&serialized) {
        Ok(p) => p,
        Err(..) => return,
    };
    assert_eq!(value, value2);
}

#[inline(always)]
pub fn fuzz_serde_yaml_read(data: &[u8]) {
    let _ = serde_yaml::from_slice::<serde_yaml::Value>(data);
}

#[inline(always)]
pub fn fuzz_serde_yaml_read_write_read(data: &[u8]) {
    let value = match serde_yaml::from_slice::<serde_yaml::Value>(data) {
        Ok(v) => v,
        Err(_) => return,
    };
    let serialized = match serde_yaml::to_vec(&value) {
        Ok(s) => s,
        Err(_) => return,
    };
    if let Ok(v) = serde_yaml::from_slice::<serde_yaml::Value>(&serialized) {
        assert_eq!(v, value);
    }
}

#[inline(always)]
pub fn fuzz_tar_read(data: &[u8]) {
    use std::io::Cursor;

    let cursor = Cursor::new(data);
    let mut archive = tar::Archive::new(cursor);
    let entries = match archive.entries() {
        Ok(entries) => entries,
        Err(..) => return,
    };
    for entry in entries {
        if let Ok(mut entry) = entry {
            let _ = entry.path();
            let _ = entry.path_bytes();
            let _ = entry.link_name();
            let _ = entry.link_name_bytes();
            if let Ok(Some(mut extensions)) = entry.pax_extensions() {
                while let Some(Ok(extension)) = extensions.next() {
                    let _ = extension.key();
                    let _ = extension.key_bytes();
                    let _ = extension.value();
                    let _ = extension.value_bytes();
                }
            }
            let _ = entry.header().as_old();
            let _ = entry.header().as_ustar();
            let _ = entry.header().as_gnu();
            let _ = entry.header().as_bytes();
            let _ = entry.header().entry_size();
            let _ = entry.header().size();
            let _ = entry.header().path();
            let _ = entry.header().path_bytes();
            let _ = entry.header().link_name();
            let _ = entry.header().link_name_bytes();
            let _ = entry.header().mode();
            let _ = entry.header().uid();
            let _ = entry.header().gid();
            let _ = entry.header().mtime();
            let _ = entry.header().username();
            let _ = entry.header().username_bytes();
            let _ = entry.header().groupname();
            let _ = entry.header().groupname_bytes();
            let _ = entry.header().device_major();
            let _ = entry.header().device_minor();
            let _ = entry.header().entry_type();
            let _ = entry.header().cksum();
        }
    }
}

fn tar_roundtrip(data: &[u8]) -> std::io::Result<()> {
    use std::io::{Read, Cursor};

    let mut output = Vec::with_capacity(data.len());
    {
        let mut archive = tar::Archive::new(Cursor::new(data));
        let mut builder = tar::Builder::new(&mut output);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).unwrap();
            builder.append(entry.header(), Cursor::new(buf))?;
        }
        builder.finish().unwrap();
    }


    {
        let mut original = tar::Archive::new(Cursor::new(data));
        let mut output = tar::Archive::new(Cursor::new(&output));

        let iter = original
            .entries()
            .unwrap()
            .zip(output.entries().unwrap());

        for (e1, e2) in iter {
            let mut e1 = e1.unwrap();
            let mut e2 = e2.unwrap();

            // File data is the same
            let mut b1 = Vec::new();
            let mut b2 = Vec::new();
            e1.read_to_end(&mut b1).unwrap();
            e2.read_to_end(&mut b2).unwrap();
            assert_eq!(b1, b2);

            // headers are the same
            let h1 = e1.header().as_bytes();
            let h2 = e2.header().as_bytes();
            assert!(h1.iter()
                .zip(h2.iter())
                .all(|(a, b)| a == b));
        }
    }
    Ok(())
}

#[inline(always)]
pub fn fuzz_tar_roundtrip(data: &[u8]) {
    let _ = tar_roundtrip(data);
}

#[inline(always)]
pub fn fuzz_toml_roundtrip(data: &[u8]) {
    if let Ok(data) = toml::from_slice::<toml::Value>(data) {
        let s = toml::to_string(&data).unwrap();
        let copy = toml::from_str(&s).unwrap();
        assert_eq!(data, copy);
    }
}

#[inline(always)]
pub fn fuzz_url_read(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = url::Url::parse(s);
    }
}

#[inline(always)]
pub fn fuzz_uuid_read(data: &[u8]) {
    use uuid::Uuid;

    if let Ok(data) = std::str::from_utf8(data) {
        let _ = Uuid::parse_str(data);
    } else {
        let _ = Uuid::from_bytes(data);
    }
}

#[inline(always)]
pub fn fuzz_xml_read(data: &[u8]) {
    let reader = xml::reader::EventReader::new(data);
    for _ in reader.into_iter() { }
}

#[inline(always)]
pub fn fuzz_zip_read(data: &[u8]) {
    let reader = std::io::Cursor::new(data);
    let mut archive = if let Ok(x) = zip::ZipArchive::new(reader) { x } else { return; };

    for i in 0..archive.len() {
        use std::io::prelude::*;

        let file = archive.by_index(i).unwrap();
        let _size = file.bytes().count();
    }
}

#[inline(always)]
pub fn fuzz_zopfli_compress(data: &[u8]) {
    let options = zopfli::Options::default();

    for output_type in &[
        zopfli::Format::Deflate,
        zopfli::Format::Gzip,
        zopfli::Format::Zlib,
    ] {
        let mut res = Vec::with_capacity(data.len() / 2);
        let _ = zopfli::compress(&options, &output_type, &data, &mut res);
    }
}