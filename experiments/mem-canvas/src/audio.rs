
use macroquad::audio::{Sound, load_sound_from_bytes, play_sound, PlaySoundParams};

pub struct AudioManager {
    pub alloc_sounds: Vec<Sound>,
    pub free_sound: Option<Sound>,
}

impl AudioManager {
    pub async fn new() -> Self {
        let mut alloc_sounds = Vec::new();
        // Generate 5 pitch variations
        let freqs = [220.0, 330.0, 440.0, 550.0, 660.0];

        for &freq in &freqs {
            let wav = generate_sine_wave(freq, 0.1);
            if let Ok(sound) = load_sound_from_bytes(&wav).await {
                alloc_sounds.push(sound);
            }
        }

        let free_wav = generate_noise(0.15); // Slightly longer noise
        let free_sound = load_sound_from_bytes(&free_wav).await.ok();

        if alloc_sounds.is_empty() {
            println!("Warning: Failed to load alloc sounds");
        }

        AudioManager {
            alloc_sounds,
            free_sound,
        }
    }

    pub fn play_alloc(&self, size: usize) {
        if self.alloc_sounds.is_empty() { return; }

        // Map size to index (smaller size -> higher pitch index?)
        // Let's say size 10 -> index 4 (660Hz)
        // size 1000 -> index 0 (220Hz)

        // Invert mapping: higher index = higher freq.
        // We want small blocks to be high pitch.
        let max_size = 1000.0;
        let norm_size = (size as f32).min(max_size) / max_size; // 0.0 - 1.0

        // index = (1.0 - norm_size) * (len - 1)
        let idx = ((1.0 - norm_size) * (self.alloc_sounds.len() as f32 - 1.0)).round() as usize;
        let idx = idx.clamp(0, self.alloc_sounds.len() - 1);

        let sound = &self.alloc_sounds[idx];
        play_sound(sound, PlaySoundParams {
            looped: false,
            volume: 0.5,
        });
    }

    pub fn play_free(&self, _size: usize) {
        if let Some(sound) = &self.free_sound {
             play_sound(sound, PlaySoundParams {
                looped: false,
                volume: 0.3,
            });
        }
    }
}

fn generate_sine_wave(freq: f32, duration: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let value = (t * freq * 2.0 * std::f32::consts::PI).sin();
        // Envelope
        let envelope = if t < 0.01 {
            t / 0.01
        } else if t > duration - 0.01 {
            (duration - t) / 0.01
        } else {
            1.0
        };

        let sample = (value * envelope * i16::MAX as f32 * 0.8) as i16;
        data.extend_from_slice(&sample.to_le_bytes());
    }

    create_wav_file(&data, sample_rate, 1)
}

fn generate_noise(duration: f32) -> Vec<u8> {
    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let value: f32 = rand::random::<f32>() * 2.0 - 1.0;
        let envelope = if t < 0.01 {
            t / 0.01
        } else if t > duration - 0.01 {
            (duration - t) / 0.01
        } else {
            1.0
        };
        let sample = (value * envelope * i16::MAX as f32 * 0.5) as i16;
        data.extend_from_slice(&sample.to_le_bytes());
    }

    create_wav_file(&data, sample_rate, 1)
}

fn create_wav_file(data: &[u8], sample_rate: u32, channels: u16) -> Vec<u8> {
    let mut buffer = Vec::new();
    let data_len = data.len() as u32;
    let file_len = 36 + data_len;
    let byte_rate = sample_rate * channels as u32 * 2; // 16 bit
    let block_align = channels * 2;

    // RIFF
    buffer.extend_from_slice(b"RIFF");
    buffer.extend_from_slice(&file_len.to_le_bytes());
    buffer.extend_from_slice(b"WAVE");

    // fmt
    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&(16u32).to_le_bytes());
    buffer.extend_from_slice(&(1u16).to_le_bytes());
    buffer.extend_from_slice(&channels.to_le_bytes());
    buffer.extend_from_slice(&sample_rate.to_le_bytes());
    buffer.extend_from_slice(&byte_rate.to_le_bytes());
    buffer.extend_from_slice(&block_align.to_le_bytes());
    buffer.extend_from_slice(&(16u16).to_le_bytes());

    // data
    buffer.extend_from_slice(b"data");
    buffer.extend_from_slice(&data_len.to_le_bytes());
    buffer.extend_from_slice(data);

    buffer
}
