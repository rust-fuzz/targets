#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate semver;

fuzz_target!(|data: &[u8]| {
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
});
