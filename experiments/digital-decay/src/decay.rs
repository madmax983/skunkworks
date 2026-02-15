use rand::Rng;

/// Simulates bit rot on a byte slice.
/// `severity` is the probability (0.0 to 1.0) that a byte will be corrupted.
pub fn bit_rot(data: &mut [u8], severity: f32) {
    let mut rng = rand::thread_rng();
    for byte in data.iter_mut() {
        if rng.gen::<f32>() < severity {
            // Flip a random bit
            let bit_idx = rng.gen_range(0..8);
            *byte ^= 1 << bit_idx;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_rot_modifies_data() {
        let original = vec![0u8; 1000];
        let mut corrupted = original.clone();

        // Apply high severity bit rot
        bit_rot(&mut corrupted, 0.5);

        // It's statistically impossible for 1000 bytes to remain unchanged with 0.5 probability
        assert_ne!(original, corrupted);

        // Verify length is preserved
        assert_eq!(original.len(), corrupted.len());
    }

    #[test]
    fn test_bit_rot_zero_severity() {
        let original = vec![0u8; 1000];
        let mut corrupted = original.clone();

        bit_rot(&mut corrupted, 0.0);

        assert_eq!(original, corrupted);
    }
}
