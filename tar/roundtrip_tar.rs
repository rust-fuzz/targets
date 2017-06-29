#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate tar;

use std::io::{Read, Cursor};

fuzz_target!(|data| {
    let _ = process(data);
});


fn process(data: &[u8]) -> std::io::Result<()> {
    let mut output = Vec::with_capacity(data.len());
    {
        let mut archive = tar::Archive::new(Cursor::new(data));
        let mut builder = tar::Builder::new(&mut output);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).unwrap();
            builder.append(entry.header(), Cursor::new(buf));
        }
        builder.finish().unwrap();
    }


    {
        let mut original = tar::Archive::new(Cursor::new(data));
        let mut output = tar::Archive::new(Cursor::new(&output));

        let mut iter = original
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
