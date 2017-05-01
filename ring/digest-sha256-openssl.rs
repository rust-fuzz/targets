#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate ring;
extern crate openssl;

fuzz_target!(|data: &[u8]| {
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
});
