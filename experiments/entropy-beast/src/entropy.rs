use rand::Rng;

pub fn corrupt(data: &mut Vec<u8>, intensity: f32) {
    if intensity <= 0.0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let len = data.len();
    if len == 0 {
        return;
    }

    // Number of mutations scales with intensity and length
    // e.g. intensity 0.1 means ~1 mutation per 100 bytes?
    // Let's say intensity is probability per byte.
    // If intensity is 0.01, 1% of bytes are affected.

    let mutations = (len as f32 * intensity).ceil() as usize;

    for _ in 0..mutations {
        let mutation_type = rng.gen_range(0..5);
        let idx = rng.gen_range(0..len);

        match mutation_type {
            0 => {
                // Bit Flip
                if idx < data.len() {
                    let bit = rng.gen_range(0..8);
                    data[idx] ^= 1 << bit;
                }
            }
            1 => {
                // Byte Swap
                if idx + 1 < data.len() {
                    data.swap(idx, idx + 1);
                }
            }
            2 => {
                // Byte Drop (rare, very destructive)
                if rng.gen_bool(0.1) && data.len() > 10 {
                    if idx < data.len() {
                         data.remove(idx);
                    }
                }
            }
            3 => {
                // Byte Insert (rare)
                if rng.gen_bool(0.1) {
                    if idx <= data.len() {
                        data.insert(idx, rng.gen());
                    }
                }
            }
            4 => {
                // Zero Range (small range)
                let range_len = rng.gen_range(1..4);
                for i in 0..range_len {
                    if idx + i < data.len() {
                        data[idx + i] = 0;
                    }
                }
            }
            _ => {}
        }
    }
}
