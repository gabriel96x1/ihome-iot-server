use hound::WavReader;

pub fn read_wav(path: &str) -> Vec<i16> {
    let mut reader = WavReader::open(path).expect("Cannot open WAV file");
    reader.samples::<i16>().map(|s| s.unwrap()).collect()
}

pub fn save_wav(path: &str, samples: &[i16]) -> String {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for s in samples {
        writer.write_sample(*s).unwrap();
    }
    writer.finalize().unwrap();
    println!("Audio saved at {}", path);

    let speech_path: String = path.into();

    speech_path
}