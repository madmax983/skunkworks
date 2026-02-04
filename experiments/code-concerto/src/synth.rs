use hound;
use crate::mapping::MusicalEvent;
use std::f32::consts::PI;

pub struct Synthesizer {
    sample_rate: u32,
}

impl Synthesizer {
    pub fn new() -> Self {
        Self { sample_rate: 44100 }
    }

    pub fn write_wav(&self, events: &[MusicalEvent], filename: &str) -> anyhow::Result<()> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(filename, spec)?;

        for event in events {
            self.synthesize_event(&mut writer, event)?;
        }

        writer.finalize()?;
        Ok(())
    }

    fn synthesize_event<W>(&self, writer: &mut hound::WavWriter<W>, event: &MusicalEvent) -> anyhow::Result<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        match event {
            MusicalEvent::Note { freq, duration } => {
                self.write_tone(writer, vec![*freq], *duration)?;
            }
            MusicalEvent::Chord { freqs, duration } => {
                self.write_tone(writer, freqs.clone(), *duration)?;
            }
            MusicalEvent::Rest { duration } => {
                let num_samples = (self.sample_rate as f32 * duration) as u32;
                for _ in 0..num_samples {
                    writer.write_sample(0_i16)?;
                }
            }
        }
        Ok(())
    }

    fn write_tone<W>(&self, writer: &mut hound::WavWriter<W>, freqs: Vec<f32>, duration: f32) -> anyhow::Result<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        let num_samples = (self.sample_rate as f32 * duration) as u32;
        let amplitude = i16::MAX as f32 * 0.5; // 50% volume to avoid clipping on chords

        for t in 0..num_samples {
            let time = t as f32 / self.sample_rate as f32;
            let mut sample_val = 0.0;

            for freq in &freqs {
                sample_val += (2.0 * PI * freq * time).sin();
            }

            // Average amplitude
            if !freqs.is_empty() {
                sample_val /= freqs.len() as f32;
            }

            // Envelope (simple attack/release)
            let envelope = if t < 1000 {
                t as f32 / 1000.0
            } else if t > num_samples - 1000 {
                (num_samples - t) as f32 / 1000.0
            } else {
                1.0
            };

            let final_sample = (sample_val * amplitude * envelope) as i16;
            writer.write_sample(final_sample)?;
        }
        Ok(())
    }
}
