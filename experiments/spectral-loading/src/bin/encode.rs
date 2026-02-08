use std::env;
use std::fs;
use spectral_loading::codec::{encode, SAMPLE_RATE};
use hound;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_payload> <output.wav>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let payload = fs::read(input_path)?;
    println!("Reading payload from {}: {} bytes", input_path, payload.len());

    let audio_samples = encode(&payload);
    println!("Generated {} audio samples", audio_samples.len());

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, spec)?;

    let mut clipped = 0;
    for &sample in &audio_samples {
        // Hard clip to avoid overflow
        let clamped = sample.max(-1.0).min(1.0);
        if clamped != sample {
            clipped += 1;
        }
        let amplitude = (clamped * 32767.0) as i16;
        writer.write_sample(amplitude)?;
    }

    writer.finalize()?;

    if clipped > 0 {
        println!("Warning: {} samples clipped!", clipped);
    }

    println!("Written to {}", output_path);
    Ok(())
}
