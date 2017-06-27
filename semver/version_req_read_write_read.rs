#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate semver;

fuzz_target!(|data: &[u8]| {
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
});
