use rustfft::{FftPlanner, num_complex::Complex};
use rand::Rng;

pub const SAMPLE_RATE: u32 = 44100;
pub const WINDOW_SIZE: usize = 2048;
pub const DATA_BIN_START: usize = 700;
pub const DATA_BINS: usize = 256;
pub const SPECTRAL_MAGNITUDE: f32 = 20.0;
pub const THRESHOLD: f32 = 10.0;

pub fn encode(data: &[u8]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(WINDOW_SIZE);

    let bytes_per_frame = DATA_BINS / 8;

    // Add length header (4 bytes, Little Endian)
    let len_bytes = (data.len() as u32).to_le_bytes();
    let mut full_data = Vec::with_capacity(4 + data.len());
    full_data.extend_from_slice(&len_bytes);
    full_data.extend_from_slice(data);

    // Padding with zeros if needed to fill last frame is implicit by chunking logic?
    // Iterating by chunks will leave remainder. We must handle it.

    let mut output_audio = Vec::new();
    let mut scratch = vec![Complex::new(0.0, 0.0); WINDOW_SIZE];

    // Iterate in full chunks, handle last partial
    let chunks: Vec<&[u8]> = full_data.chunks(bytes_per_frame).collect();

    for chunk in chunks {
        let mut spectrum = vec![Complex::new(0.0, 0.0); WINDOW_SIZE];
        let mut rng = rand::thread_rng();

        for (i, &byte) in chunk.iter().enumerate() {
            for bit in 0..8 {
                if (byte >> bit) & 1 == 1 {
                    let bin = DATA_BIN_START + i * 8 + bit;
                    let phase = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
                    let val = Complex::new(
                        SPECTRAL_MAGNITUDE * phase.cos(),
                        SPECTRAL_MAGNITUDE * phase.sin()
                    );
                    spectrum[bin] = val;
                }
            }
        }

        // Conjugate symmetry for bins 1..N/2
        // N/2 is 1024.
        // If bin 1023 is set, bin 1025 must be conjugate.
        // spectrum[N - k] = spectrum[k].conj()
        for k in 1..WINDOW_SIZE/2 {
            let val = spectrum[k];
            if val.norm_sqr() > 0.0 {
                spectrum[WINDOW_SIZE - k] = val.conj();
            }
        }
        // What about k > N/2? We only set up to 956 < 1024. So we are fine.

        fft.process_with_scratch(&mut spectrum, &mut scratch);

        let scale = 1.0 / WINDOW_SIZE as f32;
        for c in spectrum.iter() {
            output_audio.push(c.re * scale);
        }
    }

    output_audio
}

pub fn decode(audio: &[f32]) -> Option<Vec<u8>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);

    let mut decoded_bytes = Vec::new();
    let mut scratch = vec![Complex::new(0.0, 0.0); WINDOW_SIZE];

    for window in audio.chunks(WINDOW_SIZE) {
        if window.len() < WINDOW_SIZE { break; }

        let mut input: Vec<Complex<f32>> = window.iter().map(|&s| Complex::new(s, 0.0)).collect();
        fft.process_with_scratch(&mut input, &mut scratch);

        // Read bits
        let mut frame_bytes = Vec::new();
        let mut current_byte = 0u8;

        for i in 0..DATA_BINS {
            let bin = DATA_BIN_START + i;
            let mag = input[bin].norm();

            if mag > THRESHOLD {
                let bit = i % 8;
                current_byte |= 1 << bit;
            }

            if (i + 1) % 8 == 0 {
                frame_bytes.push(current_byte);
                current_byte = 0;
            }
        }
        decoded_bytes.extend_from_slice(&frame_bytes);
    }

    if decoded_bytes.len() < 4 {
        return None;
    }

    let len = u32::from_le_bytes([decoded_bytes[0], decoded_bytes[1], decoded_bytes[2], decoded_bytes[3]]) as usize;

    if decoded_bytes.len() < 4 + len {
        // Not enough data
        return None;
    }

    Some(decoded_bytes[4..4+len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let payload = b"Function(x) { return x * 2; }";
        let audio = encode(payload);

        // Noise?
        // Let's add some noise
        let noisy_audio: Vec<f32> = audio.iter().map(|&s| s + (rand::random::<f32>() - 0.5) * 0.01).collect();

        let decoded = decode(&noisy_audio);
        assert!(decoded.is_some(), "Failed to decode");
        let decoded_bytes = decoded.unwrap();
        assert_eq!(decoded_bytes, payload);
    }

    #[test]
    fn test_amplitude() {
        // Verify that the generated audio is within [-1.0, 1.0] reasonable bounds
        // (It can clip, but we want to know if it's massively distorted)
        let payload = vec![0xFF; 100]; // Lots of 1s
        let audio = encode(&payload);
        let max = audio.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        println!("Max amplitude: {}", max);
        assert!(max < 2.0, "Amplitude too high, might clip heavily in WAV");
    }
}
