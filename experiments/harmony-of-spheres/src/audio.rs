use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
}

pub struct AudioEngine {
    // Map (Waveform, Frequency) to Sound
    notes: BTreeMap<(Waveform, u32), Sound>,
}

impl AudioEngine {
    pub async fn new() -> Self {
        let mut notes = BTreeMap::new();

        // Chromatic Scale from C2 (65.41) to C7 (2093.0)
        let mut freqs = Vec::new();
        let mut f = 65.41;
        while f <= 2100.0 {
            freqs.push(f);
            f *= 1.059463; // 2^(1/12)
        }

        // Generate sounds for each waveform and frequency
        for &freq in &freqs {
            // Sine
            let sine_wav = generate_wave(freq, Waveform::Sine, 0.5);
            if let Ok(sound) = load_sound_from_bytes(&sine_wav).await {
                notes.insert((Waveform::Sine, freq as u32), sound);
            }

            // Square (Bass range mostly)
            if freq < 500.0 {
                let square_wav = generate_wave(freq, Waveform::Square, 0.4);
                if let Ok(sound) = load_sound_from_bytes(&square_wav).await {
                    notes.insert((Waveform::Square, freq as u32), sound);
                }
            }

            // Saw (Mid-High range)
            if freq > 200.0 {
                let saw_wav = generate_wave(freq, Waveform::Saw, 0.4);
                if let Ok(sound) = load_sound_from_bytes(&saw_wav).await {
                    notes.insert((Waveform::Saw, freq as u32), sound);
                }
            }
        }

        Self { notes }
    }

    pub fn play_note(&self, freq: f32, waveform: Waveform, volume: f32) {
        if self.notes.is_empty() {
            return;
        }

        // Find closest frequency for this waveform
        let target = freq as u32;
        let mut closest_freq = 0;
        let mut min_diff = u32::MAX;
        let mut found = false;

        // Iterate over keys matching waveform
        // BTreeMap is sorted, so we can iterate range? But (Waveform, Freq) is the key.
        // We can iterate range (Waveform, 0)..(Waveform, MAX).

        // Simple linear search over the subset (or use range)
        for (&(w, f), _) in self.notes.range((waveform, 0)..(waveform, u32::MAX)) {
            if w != waveform {
                continue;
            } // Should be covered by range
            let diff = f.abs_diff(target);
            if diff < min_diff {
                min_diff = diff;
                closest_freq = f;
                found = true;
            }
        }

        if found {
            if let Some(sound) = self.notes.get(&(waveform, closest_freq)) {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped: false,
                        volume,
                    },
                );
            }
        } else {
            // Fallback to Sine if requested waveform not available
            if waveform != Waveform::Sine {
                self.play_note(freq, Waveform::Sine, volume);
            }
        }
    }
}

fn generate_wave(freq: f32, waveform: Waveform, duration_secs: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut buffer = Vec::with_capacity(44 + num_samples * 2);

    // WAV Header
    buffer.extend_from_slice(b"RIFF");
    let file_size = 36 + num_samples * 2;
    buffer.extend_from_slice(&(file_size as u32).to_le_bytes());
    buffer.extend_from_slice(b"WAVE");

    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&(16u32).to_le_bytes()); // Chunk size
    buffer.extend_from_slice(&(1u16).to_le_bytes()); // PCM
    buffer.extend_from_slice(&(1u16).to_le_bytes()); // Channels (Mono)
    buffer.extend_from_slice(&(sample_rate as u32).to_le_bytes()); // Sample rate
    let byte_rate = sample_rate * 2;
    buffer.extend_from_slice(&(byte_rate as u32).to_le_bytes());
    buffer.extend_from_slice(&(2u16).to_le_bytes()); // Block align
    buffer.extend_from_slice(&(16u16).to_le_bytes()); // Bits per sample

    buffer.extend_from_slice(b"data");
    let data_size = num_samples * 2;
    buffer.extend_from_slice(&(data_size as u32).to_le_bytes());

    // Samples
    let mut phase = 0.0;
    let phase_inc = freq / sample_rate as f32;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = (1.0 - t / duration_secs).powf(2.0); // Simple quadratic decay

        let sample_val = match waveform {
            Waveform::Sine => (phase * 2.0 * std::f32::consts::PI).sin(),
            Waveform::Square => {
                if phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Saw => phase * 2.0 - 1.0,
        };

        phase += phase_inc;
        if phase > 1.0 {
            phase -= 1.0;
        }

        let value = (sample_val * envelope * 8000.0) as i16; // 8000 volume (conservative)
        buffer.extend_from_slice(&value.to_le_bytes());
    }

    buffer
}
