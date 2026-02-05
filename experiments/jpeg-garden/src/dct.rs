use std::f32::consts::PI;

#[allow(dead_code)]
pub const BLOCK_SIZE: usize = 8;

fn c(u: usize) -> f32 {
    if u == 0 {
        1.0 / 2.0f32.sqrt()
    } else {
        1.0
    }
}

pub fn dct_2d(input: &[f32; 64]) -> [f32; 64] {
    let mut output = [0.0; 64];

    for u in 0..8 {
        for v in 0..8 {
            let mut sum = 0.0;
            for x in 0..8 {
                for y in 0..8 {
                    let pixel = input[y * 8 + x];
                    let cos_x = ((2.0 * x as f32 + 1.0) * u as f32 * PI / 16.0).cos();
                    let cos_y = ((2.0 * y as f32 + 1.0) * v as f32 * PI / 16.0).cos();
                    sum += pixel * cos_x * cos_y;
                }
            }
            output[v * 8 + u] = 0.25 * c(u) * c(v) * sum;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dct_idct_roundtrip() {
        let mut input = [0.0; 64];
        for i in 0..64 {
            input[i] = i as f32; // Gradient
        }

        let dct = dct_2d(&input);
        let idct = idct_2d(&dct);

        for i in 0..64 {
            let diff = (input[i] - idct[i]).abs();
            assert!(diff < 0.01, "Mismatch at {}: input {}, output {}, diff {}", i, input[i], idct[i], diff);
        }
    }
}

pub fn idct_2d(input: &[f32; 64]) -> [f32; 64] {
    let mut output = [0.0; 64];

    for x in 0..8 {
        for y in 0..8 {
            let mut sum = 0.0;
            for u in 0..8 {
                for v in 0..8 {
                    let coeff = input[v * 8 + u];
                    let cos_x = ((2.0 * x as f32 + 1.0) * u as f32 * PI / 16.0).cos();
                    let cos_y = ((2.0 * y as f32 + 1.0) * v as f32 * PI / 16.0).cos();
                    sum += c(u) * c(v) * coeff * cos_x * cos_y;
                }
            }
            output[y * 8 + x] = 0.25 * sum;
        }
    }
    output
}
