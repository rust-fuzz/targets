#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate flac;

use flac::{ByteStream, Stream};

fuzz_target!(|data| {
    let s = Stream::<ByteStream>::from_buffer(data);
    if let Ok(mut stream) = s {
        let _ = stream.info();
        let _ = stream.metadata();
        let mut iter = stream.iter::<i8>();
        while iter.next().is_some() { }
    }
});

