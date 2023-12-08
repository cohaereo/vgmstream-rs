use std::env::args;
use std::fs::File;

fn main() {
    let data = std::fs::read(args().nth(1).unwrap()).unwrap();

    let (samples, desc) = vgmstream::read_file_to_samples_no_questions_asked(
        &data,
        Some(args().nth(1).unwrap().to_string()),
    )
    .expect("Failed to render file");

    println!("{desc:#?}");

    let mut out = File::create("test.wav").unwrap();

    wav::write(
        wav::Header {
            audio_format: wav::WAV_FORMAT_PCM,
            channel_count: desc.channels as u16,
            sampling_rate: desc.sample_rate as u32,
            bytes_per_second: desc.bitrate as u32,
            bytes_per_sample: 2,
            bits_per_sample: 16,
        },
        &wav::BitDepth::Sixteen(samples),
        &mut out,
    )
    .unwrap();
}
