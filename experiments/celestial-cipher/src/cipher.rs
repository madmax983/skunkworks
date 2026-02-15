use sha2::{Digest, Sha256};

pub fn generate_key(angles: &[f32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for angle in angles {
        // Normalize angle to 0..2PI
        let mut a = angle % (2.0 * std::f32::consts::PI);
        if a < 0.0 {
            a += 2.0 * std::f32::consts::PI;
        }

        // We use the raw float bytes.
        // Note: Floating point determinism across machines is tricky,
        // but for a single simulation run, it's consistent.
        // To make it robust, we could quantize: (a * 1000.0) as u32.
        let quantized = (a * 1000.0) as u32;
        hasher.update(quantized.to_le_bytes());
    }
    hasher.finalize().into()
}

pub fn encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(key.iter().cycle())
        .map(|(b, k)| b ^ k)
        .collect()
}

pub fn hex_encode(data: &[u8]) -> String {
    hex::encode(data)
}
