use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io::{Read, Write};

/// Compresses data and embeds it into the LSBs of the image.
/// The first 4 bytes (32 bits) store the length of the compressed data.
pub fn encode(image: &mut RgbaImage, data: &[u8]) -> Result<()> {
    // Compress data
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed_data = encoder.finish()?;
    let length = compressed_data.len() as u32;

    let total_bits_needed = (4 + compressed_data.len()) * 8;
    let capacity = (image.width() * image.height() * 3) as usize; // using RGB channels

    if total_bits_needed > capacity {
        return Err(anyhow!(
            "Image capacity exceeded. Needed {} bits, have {}",
            total_bits_needed,
            capacity
        ));
    }

    let mut bits = Vec::with_capacity(total_bits_needed);

    // Store length (32 bits, big endian)
    for i in (0..32).rev() {
        bits.push((length >> i) & 1);
    }

    // Store data bits
    for byte in &compressed_data {
        for i in (0..8).rev() {
            bits.push(((*byte as u32) >> i) & 1);
        }
    }

    let mut bit_idx = 0;

    // Iterate over pixels and modify LSB of R, G, B
    for pixel in image.pixels_mut() {
        if bit_idx >= bits.len() {
            break;
        }

        // Red channel
        if bit_idx < bits.len() {
            pixel[0] = (pixel[0] & 0xFE) | (bits[bit_idx] as u8);
            bit_idx += 1;
        }

        // Green channel
        if bit_idx < bits.len() {
            pixel[1] = (pixel[1] & 0xFE) | (bits[bit_idx] as u8);
            bit_idx += 1;
        }

        // Blue channel
        if bit_idx < bits.len() {
            pixel[2] = (pixel[2] & 0xFE) | (bits[bit_idx] as u8);
            bit_idx += 1;
        }
    }

    Ok(())
}

pub fn decode(image: &RgbaImage) -> Result<Vec<u8>> {
    let mut length: u32 = 0;
    let mut bit_count = 0;

    // First extract length (32 bits)
    // We scan the image exactly as we encoded it: R, then G, then B for each pixel.
    'header_scan: for pixel in image.pixels() {
        for c in 0..3 {
            length = (length << 1) | ((pixel[c] & 1) as u32);
            bit_count += 1;
            if bit_count == 32 {
                break 'header_scan;
            }
        }
    }

    // Now extract data
    let total_data_bits = (length * 8) as usize;
    let mut current_byte: u8 = 0;
    let mut bit_in_byte = 0;
    let mut extracted_bytes = Vec::with_capacity(length as usize);

    // We need to skip the first 32 bits we already read.
    // Let's re-scan carefully.
    let mut total_bits_scanned = 0;
    let start_bit = 32;
    let end_bit = 32 + total_data_bits;

    'data_scan: for pixel in image.pixels() {
        for c in 0..3 {
            if total_bits_scanned >= start_bit {
                let bit = pixel[c] & 1;
                current_byte = (current_byte << 1) | bit;
                bit_in_byte += 1;

                if bit_in_byte == 8 {
                    extracted_bytes.push(current_byte);
                    current_byte = 0;
                    bit_in_byte = 0;
                }
            }

            total_bits_scanned += 1;
            if total_bits_scanned >= end_bit {
                break 'data_scan;
            }
        }
    }

    if extracted_bytes.len() != length as usize {
        return Err(anyhow!(
            "Failed to extract all data. Image might be corrupted or cropped."
        ));
    }

    // Decompress
    let mut decoder = ZlibDecoder::new(&extracted_bytes[..]);
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;

    Ok(decoded_data)
}

pub fn generate_nebula(width: u32, height: u32, seed: u64) -> RgbaImage {
    let mut img = ImageBuffer::new(width, height);
    let mut rng = StdRng::seed_from_u64(seed);

    // Background gradient
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Deep space blue/purple
        let noise = rng.gen_range(0..15);
        let r = noise + (x as f32 / width as f32 * 20.0) as u8;
        let g = noise + (y as f32 / height as f32 * 20.0) as u8;
        let b = noise + 20 + rng.gen_range(0..10);

        *pixel = Rgba([r, g, b, 255]);
    }

    // Stars
    let num_stars = (width * height) / 100;
    for _ in 0..num_stars {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let brightness = rng.gen_range(150..255);
        img.put_pixel(x, y, Rgba([brightness, brightness, brightness, 255]));
    }

    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let mut img = generate_nebula(100, 100, 12345);
        let data = b"Hello, Stardust! This is a secret message.";
        encode(&mut img, data).unwrap();
        let decoded = decode(&img).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_capacity_check() {
        let mut img = generate_nebula(10, 10, 12345); // 100 pixels * 3 channels = 300 bits
                                                      // Use random data to ensure it doesn't compress too much
        let mut rng = StdRng::seed_from_u64(999);
        let mut data = vec![0u8; 100];
        rng.fill(&mut data[..]);
        // 100 bytes random data ~ 100 bytes compressed + header > 300 bits
        assert!(encode(&mut img, &data).is_err());
    }
}
