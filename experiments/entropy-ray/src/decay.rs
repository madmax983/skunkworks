use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecayMode {
    Gamma,  // Random bit flips
    XRay,   // Byte shifting / Structural damage
    Cosmic, // Burst errors
    Mold,   // Neighbor spread / organic growth
}

pub fn apply_decay(data: &mut [u8], mode: DecayMode, intensity: f32) -> usize {
    let len = data.len();
    if len == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    let mut bits_flipped = 0;

    // Intensity is roughly probability of an event per byte, but scaled down
    // Let's interpret intensity as a fraction of the buffer to affect.
    // e.g. 0.0001 means 1 in 10000 bytes.

    let num_events = (len as f32 * intensity).ceil() as usize;

    match mode {
        DecayMode::Gamma => {
            for _ in 0..num_events {
                let idx = rng.gen_range(0..len);
                let bit = rng.gen_range(0..8);
                let old = data[idx];
                data[idx] ^= 1 << bit;
                if data[idx] != old {
                    bits_flipped += 1;
                }
            }
        }
        DecayMode::XRay => {
            // Shift blocks of bytes
            for _ in 0..(num_events / 10).max(1) {
                let start = rng.gen_range(0..len - 1);
                let span = rng.gen_range(10..100);
                let end = (start + span).min(len);
                if end > start + 1 {
                    data[start..end].rotate_left(1);
                    bits_flipped += span; // Approximate impact
                }
            }
        }
        DecayMode::Cosmic => {
            // Zero out or set to 0xFF a block (Radiation burst)
            for _ in 0..(num_events / 20).max(1) {
                let start = rng.gen_range(0..len);
                let length = rng.gen_range(5..50);
                let end = (start + length).min(len);
                let val = if rng.gen() { 0xFF } else { 0x00 };
                for byte in &mut data[start..end] {
                    if *byte != val {
                        *byte = val;
                        bits_flipped += 1;
                    }
                }
            }
        }
        DecayMode::Mold => {
            // Copy byte to neighbor (spreading infection)
            for _ in 0..num_events {
                let idx = rng.gen_range(0..len - 1);
                if data[idx + 1] != data[idx] {
                    data[idx + 1] = data[idx];
                    bits_flipped += 1;
                }
            }
        }
    }
    bits_flipped
}
