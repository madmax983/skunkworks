use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;

pub struct AudioEngine {
    sounds: Vec<Sound>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            sounds: Vec::new(),
        }
    }

    pub async fn init(&mut self) {
        // Generate a Pentatonic Scale: C, D, E, G, A
        // Frequencies: 261.63, 293.66, 329.63, 392.00, 440.00
        // And higher octave.
        let frequencies = [
            261.63, 293.66, 329.63, 392.00, 440.00,
            523.25, 587.33, 659.25, 783.99, 880.00
        ];

        for freq in frequencies {
            let bytes = generate_sine_wave(freq, 300); // 300ms
            let sound = load_sound_from_bytes(&bytes).await;
            match sound {
                Ok(s) => self.sounds.push(s),
                Err(e) => eprintln!("Failed to load sound: {}", e),
            }
        }
    }

    pub fn play_note(&self, index: usize) {
        if self.sounds.is_empty() { return; }
        let sound = &self.sounds[index % self.sounds.len()];
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: 0.5,
            },
        );
    }
}

fn generate_sine_wave(frequency: f32, duration_ms: u32) -> Vec<u8> {
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration_ms as f32 / 1000.0) as usize;
    let mut pcm_data = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let amplitude = 0.5;

        // Envelope (ADSR-ish): Fast attack, linear decay
        let env = if t < 0.05 {
            t / 0.05
        } else {
            1.0 - (t - 0.05) / (duration_ms as f32 / 1000.0 - 0.05)
        };
        let env = env.max(0.0);

        let value = (t * frequency * 2.0 * std::f32::consts::PI).sin();
        let sample = (value * amplitude * env * 32767.0) as i16;

        // Little Endian
        pcm_data.push((sample & 0xff) as u8);
        pcm_data.push((sample >> 8) as u8);
    }

    // WAV Header
    let mut wav = Vec::new();
    // RIFF header
    wav.extend_from_slice(b"RIFF");
    let file_len = 36 + pcm_data.len() as u32;
    wav.extend_from_slice(&file_len.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav.extend_from_slice(&1u16.to_le_bytes()); // Channels (Mono)
    wav.extend_from_slice(&(sample_rate as u32).to_le_bytes()); // Sample rate
    wav.extend_from_slice(&(sample_rate as u32 * 2).to_le_bytes()); // Byte rate (SampleRate * BlockAlign)
    wav.extend_from_slice(&2u16.to_le_bytes()); // Block align (Channels * BitsPerSample / 8)
    wav.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(pcm_data.len() as u32).to_le_bytes());
    wav.extend(pcm_data);

    wav
}
