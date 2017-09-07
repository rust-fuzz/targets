// from https://github.com/PistonDevelopers/image-png/blob/69072d258487fa2d933bcdfb67d139266276ac0a/png-afl/src/main.rs

#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate png;

use png::HasParameters;

fn decode(data: &[u8]) -> Result<(png::OutputInfo, Vec<u8>), png::DecodingError> {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = decoder.read_info()?;

    if info.buffer_size() > 50_000_000 {
        return Err("Too large".to_string().into());
    }

    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data)?;

    Ok((info, img_data))
}

fn encode(info: &png::OutputInfo, data: &[u8]) -> Result<Vec<u8>, png::EncodingError> {
    let mut out = Vec::with_capacity(data.len());
    {
        let mut encoder = png::Encoder::new(&mut out, info.width, info.height);
        encoder.set(info.color_type).set(info.bit_depth);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&data)?;
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
