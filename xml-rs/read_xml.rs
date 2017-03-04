#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate xml;

fuzz_target!(|data| {
    let reader = xml::reader::EventReader::new(data);
    for _ in reader.into_iter() { }
});
