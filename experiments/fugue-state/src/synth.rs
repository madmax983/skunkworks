use crate::parser::{MusicalEvent, Timbre};
use hound;
use std::f32::consts::PI;

pub fn generate_wav(events: &[MusicalEvent], filepath: &str) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filepath, spec)?;
    let sample_rate = spec.sample_rate as f32;
    let amplitude = i16::MAX as f32 * 0.3; // 30% volume

    for event in events {
        let num_samples = (event.duration * sample_rate) as u32;

        for i in 0..num_samples {
            let t = i as f32 / sample_rate;
            let sample_value = match event.timbre {
                Timbre::Sine => (t * event.frequency * 2.0 * PI).sin(),
                Timbre::Square => (t * event.frequency * 2.0 * PI).sin().signum(),
                Timbre::Triangle => {
                    // 2 * asin(sin(2pi * freq * t)) / pi
                    (t * event.frequency * 2.0 * PI).sin().asin() * 2.0 / PI
                }
                Timbre::Sawtooth => {
                    // 2 * (t * freq - floor(0.5 + t * freq))
                    2.0 * (t * event.frequency - (t * event.frequency + 0.5).floor())
                }
            };

            // Simple envelope to avoid clicks (fade in/out 5ms)
            let envelope = if i < 220 {
                i as f32 / 220.0
            } else if i > num_samples - 220 {
                (num_samples - i) as f32 / 220.0
            } else {
                1.0
            };

            let sample = (sample_value * amplitude * envelope) as i16;
            writer.write_sample(sample)?;
        }
    }

    writer.finalize()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Timbre;
    use std::fs;

    #[test]
    fn test_generate_wav_file() {
        let events = vec![MusicalEvent {
            frequency: 440.0,
            duration: 0.1,
            timbre: Timbre::Sine,
            description: "Test".to_string(),
            #[cfg(feature = "nova")]
            span: None,
        }];
        let filename = "test_output.wav";

        generate_wav(&events, filename).unwrap();

        let metadata = fs::metadata(filename).expect("File should exist");
        assert!(metadata.len() > 44, "WAV file should have header + data");

        fs::remove_file(filename).unwrap();
    }
}
