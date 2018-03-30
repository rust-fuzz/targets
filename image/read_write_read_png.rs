// from https://github.com/PistonDevelopers/image-png/blob/69072d258487fa2d933bcdfb67d139266276ac0a/png-afl/src/main.rs

#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate png;

use png::HasParameters;

fn decode(data: &[u8]) -> Result<(png::OutputInfo, Vec<u8>), ()> {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = decoder.read_info().map_err(|_| ())?;

    if info.buffer_size() > 50_000_000 {
        return Err(());
    }

    let mut img_data = Vec::with_capacity(info.buffer_size());
    reader.next_frame(&mut img_data).map_err(|_| ())?;

    Ok((info, img_data))
}

fn encode(info: &png::OutputInfo, data: &[u8]) -> Result<Vec<u8>, ()> {
    let mut out = Vec::with_capacity(data.len());

    {
        let mut encoder = png::Encoder::new(&mut out, info.width, info.height);
        encoder.set(info.color_type).set(info.bit_depth);
        let mut writer = encoder.write_header().map_err(|_| ())?;
        writer.write_image_data(&data).map_err(|_| ())?;
    }

    Ok(out)
}

fuzz_target!(|data| {
    if let Ok((info, pixels)) = decode(data) {
        let encoded = encode(&info, &pixels).expect("encode fail");
        let (_info2, pixels2) = decode(&encoded).expect("decode failed");

        assert_eq!(pixels, pixels2);
    }
});
