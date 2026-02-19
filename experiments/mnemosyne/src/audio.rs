use image::RgbaImage;
use std::f32::consts::PI;
use rand::Rng;

pub fn generate_drone(image: &RgbaImage) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    // 1. Analyze Image (Sparse Sampling)
    let (width, height) = image.dimensions();
    let mut total_brightness = 0.0;
    let samples = 500;

    // Simple random sampling
    for _ in 0..samples {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let p = image.get_pixel(x, y);
        let b = (p[0] as f32 + p[1] as f32 + p[2] as f32) / 3.0;
        total_brightness += b;
    }
    let avg_brightness = total_brightness / samples as f32; // 0..255

    // 2. Audio Parameters
    // Brightness -> Frequency (Brighter = Higher)
    // Dark/Low Contrast -> Noise (Simulating decay/static)
    // Actually let's map:
    // Frequency: 60Hz (Hum) to 600Hz
    let base_freq = 60.0 + (avg_brightness / 255.0) * 540.0;

    // Noise increases as brightness drops (entropy/void takes over)
    let noise_mix = ((255.0 - avg_brightness) / 255.0).powf(2.0);

    let duration_secs = 0.2; // Short blip
    let sample_rate: u32 = 44100;
    let num_samples = (duration_secs * sample_rate as f32) as usize;

    let mut data = Vec::with_capacity(44 + num_samples * 2);

    // WAV Header
    data.extend_from_slice(b"RIFF");
    let file_size = 36 + num_samples * 2;
    data.extend_from_slice(&(file_size as u32).to_le_bytes());
    data.extend_from_slice(b"WAVE");
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&16u32.to_le_bytes()); // Chunk size
    data.extend_from_slice(&1u16.to_le_bytes()); // PCM
    data.extend_from_slice(&1u16.to_le_bytes()); // Mono
    data.extend_from_slice(&sample_rate.to_le_bytes());
    data.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // Byte rate
    data.extend_from_slice(&2u16.to_le_bytes()); // Block align
    data.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample
    data.extend_from_slice(b"data");
    data.extend_from_slice(&(num_samples as u32 * 2).to_le_bytes());

    // PCM Data
    let mut phase: f32 = 0.0;
    for _ in 0..num_samples {
        // Simple FM synthesis or just Sine + Noise
        // Let's add some wobble to freq
        let wobble = (phase * 10.0).sin() * 20.0;
        let freq = base_freq + wobble;

        phase += freq * 2.0 * PI / sample_rate as f32;
        if phase > 2.0 * PI { phase -= 2.0 * PI; }

        let sine = phase.sin();
        let noise: f32 = rng.gen_range(-1.0..1.0);

        // Mix
        let signal = sine * (1.0 - noise_mix) + noise * noise_mix;

        // Soft clipping
        let signal = signal.clamp(-1.0, 1.0) * 0.5; // 50% volume

        let sample_i16 = (signal * 32767.0) as i16;
        data.extend_from_slice(&sample_i16.to_le_bytes());
    }

    data
}
