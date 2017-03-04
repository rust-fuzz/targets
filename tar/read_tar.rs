#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate tar;

use std::io::Cursor;

fuzz_target!(|data| {
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
});
