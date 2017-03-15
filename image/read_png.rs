// from https://github.com/PistonDevelopers/image-png/blob/69072d258487fa2d933bcdfb67d139266276ac0a/png-afl/src/main.rs

#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate png;

fn decode(data: &[u8]) -> Result<(), png::DecodingError> {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = try!(decoder.read_info());

    if info.buffer_size() > 50_000_000 {
        return Ok(());
    }

    let mut img_data = vec![0; info.buffer_size()];
    let _ = reader.next_frame(&mut img_data);

    Ok(())
}

fuzz_target!(|data| { let _ = decode(data); });
