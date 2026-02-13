use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};
use std::collections::BTreeMap;

pub struct AudioEngine {
    notes: BTreeMap<u32, Sound>,
}

impl AudioEngine {
    pub async fn new() -> Self {
        let mut notes = BTreeMap::new();
        let base_freqs = [261.63, 293.66, 329.63, 392.00, 440.00];

        for octave in [0.5, 1.0, 2.0] {
            for &f in &base_freqs {
                let freq = f * octave;
                let wav_data = generate_sine_wave(freq, 0.5);
                if let Ok(sound) = load_sound_from_bytes(&wav_data).await {
                    notes.insert(freq as u32, sound);
                }
            }
        }
        Self { notes }
    }

    pub fn play_freq(&self, freq: f32) {
        if self.notes.is_empty() { return; }
        let target = freq as u32;
        let mut closest_freq = 0;
        let mut min_diff = u32::MAX;
        for &k in self.notes.keys() {
            let diff = k.abs_diff(target);
            if diff < min_diff {
                min_diff = diff;
                closest_freq = k;
            }
        }
        if let Some(sound) = self.notes.get(&closest_freq) {
            play_sound_once(sound);
        }
    }
}

fn generate_sine_wave(freq: f32, duration_secs: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut buffer = Vec::with_capacity(44 + num_samples * 2);

    buffer.extend_from_slice(b"RIFF");
    let file_size = 36 + num_samples * 2;
    buffer.extend_from_slice(&(file_size as u32).to_le_bytes());
    buffer.extend_from_slice(b"WAVE");
    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&(16u32).to_le_bytes());
    buffer.extend_from_slice(&(1u16).to_le_bytes());
    buffer.extend_from_slice(&(1u16).to_le_bytes());
    buffer.extend_from_slice(&(sample_rate as u32).to_le_bytes());
    let byte_rate = sample_rate * 2;
    buffer.extend_from_slice(&(byte_rate as u32).to_le_bytes());
    buffer.extend_from_slice(&(2u16).to_le_bytes());
    buffer.extend_from_slice(&(16u16).to_le_bytes());
    buffer.extend_from_slice(b"data");
    let data_size = num_samples * 2;
    buffer.extend_from_slice(&(data_size as u32).to_le_bytes());

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = (1.0 - t / duration_secs).powf(2.0);
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let value = (sample * envelope * 10000.0) as i16;
        buffer.extend_from_slice(&value.to_le_bytes());
    }
    buffer
}
