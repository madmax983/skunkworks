use image::RgbaImage;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub fn embed(image: &mut RgbaImage, data: &[u8], seed: u64) -> Result<(), String> {
    let width = image.width();
    let height = image.height();
    let total_pixels = (width * height) as usize;
    let total_channels = total_pixels * 3; // RGB only (we leave Alpha alone)

    let needed_bits = 32 + data.len() * 8;
    if needed_bits > total_channels {
        return Err(format!(
            "Payload too large for image. Needed {} bits, have {} slots.",
            needed_bits, total_channels
        ));
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..total_pixels).collect();
    indices.shuffle(&mut rng);

    // Prepare bit stream: length (32 bits, little endian) + data
    let mut bits: Vec<u8> = Vec::with_capacity(needed_bits);
    let len_u32 = data.len() as u32;
    for i in 0..32 {
        bits.push(((len_u32 >> i) & 1) as u8);
    }
    for &byte in data {
        for i in 0..8 {
            bits.push(((byte >> i) & 1) as u8);
        }
    }

    let mut bit_idx = 0;
    for pixel_idx in indices {
        if bit_idx >= bits.len() {
            break;
        }

        let x = (pixel_idx as u32) % width;
        let y = (pixel_idx as u32) / width;
        let pixel = image.get_pixel_mut(x, y);

        // Modify RGB channels (indices 0, 1, 2)
        for channel in 0..3 {
            if bit_idx < bits.len() {
                let bit = bits[bit_idx];
                pixel[channel] = (pixel[channel] & 0xFE) | bit;
                bit_idx += 1;
            }
        }
    }

    Ok(())
}

pub fn extract(image: &RgbaImage, seed: u64) -> Result<Vec<u8>, String> {
    let width = image.width();
    let height = image.height();
    let total_pixels = (width * height) as usize;

    let mut rng = StdRng::seed_from_u64(seed);
    let mut indices: Vec<usize> = (0..total_pixels).collect();
    indices.shuffle(&mut rng);

    let mut current_val: u32 = 0;
    let mut bits_read = 0;
    let mut data_bits = Vec::new();
    let mut data_len: Option<usize> = None;

    for pixel_idx in indices {
        let x = (pixel_idx as u32) % width;
        let y = (pixel_idx as u32) / width;
        let pixel = image.get_pixel(x, y);

        for channel in 0..3 {
            let bit = pixel[channel] & 1;

            if bits_read < 32 {
                // Reading length
                current_val |= (bit as u32) << bits_read;
                bits_read += 1;

                if bits_read == 32 {
                    data_len = Some(current_val as usize);
                }
            } else {
                // Reading data
                if let Some(len) = data_len {
                    if data_bits.len() < len * 8 {
                        data_bits.push(bit);
                    }
                    // If we just finished reading the last bit, we can stop early inside the channel loop
                    if data_bits.len() == len * 8 {
                        break;
                    }
                }
            }
        }

        // Break outer loop if done
        if let Some(len) = data_len {
            if data_bits.len() >= len * 8 {
                break;
            }
        }
    }

    if let Some(len) = data_len {
        if data_bits.len() < len * 8 {
            return Err("Incomplete data or corrupted length header".to_string());
        }

        let mut result = Vec::with_capacity(len);
        for chunk in data_bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                byte |= bit << i;
            }
            result.push(byte);
        }
        Ok(result)
    } else {
        Err("Failed to read length header".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn test_round_trip() {
        let mut image = RgbaImage::new(100, 100);
        // Fill with random noise to simulate real usage
        for pixel in image.pixels_mut() {
            *pixel = image::Rgba([12, 34, 56, 255]);
        }

        let payload = b"Hello, Genesis!";
        let seed = 12345;

        embed(&mut image, payload, seed).expect("Embedding failed");
        let extracted = extract(&image, seed).expect("Extraction failed");

        assert_eq!(payload.to_vec(), extracted);
    }

    #[test]
    fn test_wrong_seed() {
        let mut image = RgbaImage::new(100, 100);
        let payload = b"Secret";
        let seed_a = 12345;
        let seed_b = 67890;

        embed(&mut image, payload, seed_a).expect("Embedding failed");

        // With a wrong seed, we will likely either fail to read a valid length (huge number)
        // or read garbage data.
        // The length is read from the first 32 bits encountered by the shuffled indices.
        // Since seed B shuffles differently, it reads random bits from the image as the length.

        let result = extract(&image, seed_b);

        match result {
            Ok(data) => {
                // If by miracle it succeeds, data shouldn't match
                assert_ne!(payload.to_vec(), data);
            }
            Err(_) => {
                // Failure is also acceptable (e.g. length too big)
            }
        }
    }

    #[test]
    fn test_capacity_check() {
        let mut image = RgbaImage::new(10, 10); // 100 pixels = 300 channels = 300 bits max
                                                // payload len requires 32 bits.
                                                // remaining 268 bits / 8 = 33 bytes max.

        let payload = vec![0u8; 50]; // 50 bytes = 400 bits > 300
        let seed = 42;

        let result = embed(&mut image, &payload, seed);
        assert!(result.is_err());
    }
}
