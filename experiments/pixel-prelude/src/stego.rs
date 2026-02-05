use anyhow::{anyhow, Result};
use image::{DynamicImage, GenericImageView, GenericImage};

/// Encodes a byte slice into the LSBs of the image.
/// The format is: [Length: u32 (32 bits)] + [Data: bits]
pub fn embed_bytes(image: &mut DynamicImage, data: &[u8]) -> Result<()> {
    let (width, height) = image.dimensions();
    let total_pixels = (width * height) as usize;
    let total_channels = total_pixels * 3; // We only use R, G, B. Ignore Alpha.
    let total_capacity_bits = total_channels;

    // 4 bytes for length + data bytes
    let total_bits_needed = (4 + data.len()) * 8;

    if total_bits_needed > total_capacity_bits {
        return Err(anyhow!("Image too small. Capacity: {} bytes, Needed: {} bytes",
            total_capacity_bits / 8,
            4 + data.len()
        ));
    }

    // Prepare bit stream
    let length_bytes = (data.len() as u32).to_le_bytes();
    let mut bit_stream = Vec::with_capacity(total_bits_needed);

    // Push length bits
    for byte in length_bytes.iter() {
        for i in 0..8 {
            bit_stream.push((byte >> i) & 1);
        }
    }

    // Push data bits
    for byte in data.iter() {
        for i in 0..8 {
            bit_stream.push((byte >> i) & 1);
        }
    }

    let mut bit_idx = 0;

    for y in 0..height {
        for x in 0..width {
            if bit_idx >= bit_stream.len() {
                return Ok(());
            }

            let mut pixel = image.get_pixel(x, y);
            // pixel is usually Rgba<u8>

            // Modify R
            if bit_idx < bit_stream.len() {
                let bit = bit_stream[bit_idx];
                pixel[0] = (pixel[0] & !1) | bit;
                bit_idx += 1;
            }

            // Modify G
            if bit_idx < bit_stream.len() {
                let bit = bit_stream[bit_idx];
                pixel[1] = (pixel[1] & !1) | bit;
                bit_idx += 1;
            }

            // Modify B
            if bit_idx < bit_stream.len() {
                let bit = bit_stream[bit_idx];
                pixel[2] = (pixel[2] & !1) | bit;
                bit_idx += 1;
            }

            image.put_pixel(x, y, pixel);
        }
    }

    Ok(())
}

/// Decodes bytes from the LSBs of the image.
pub fn extract_bytes(image: &DynamicImage) -> Result<Vec<u8>> {
    let (width, height) = image.dimensions();

    // We need at least 32 bits for length
    let mut length_bits = Vec::new();
    let mut data_bits = Vec::new();
    let mut length: u32 = 0;
    let mut reading_length = true;

    for y in 0..height {
        for x in 0..width {
             let pixel = image.get_pixel(x, y);

             for channel in 0..3 { // R, G, B
                 let bit = pixel[channel] & 1;

                 if reading_length {
                     length_bits.push(bit);
                     if length_bits.len() == 32 {
                         // Reconstruct length
                         for (i, &b) in length_bits.iter().enumerate() {
                             if b == 1 {
                                 length |= 1 << i;
                             }
                         }
                         reading_length = false;

                         // Sanity check length
                         let total_pixels = (width * height) as usize;
                         let total_capacity_bits = total_pixels * 3;
                         if (length as usize * 8) + 32 > total_capacity_bits {
                             return Err(anyhow!("Corrupted header: invalid length {}", length));
                         }
                     }
                 } else {
                     data_bits.push(bit);
                     if data_bits.len() == (length as usize * 8) {
                         // Done reading
                         return bits_to_bytes(data_bits);
                     }
                 }
             }

             if !reading_length && data_bits.len() == (length as usize * 8) {
                  return bits_to_bytes(data_bits);
             }
        }
    }

    if reading_length {
        return Err(anyhow!("Image too small to contain length header"));
    }

    // If we reach here, we didn't find enough bits
    return Err(anyhow!("Unexpected end of stream. Expected {} bytes, found {} bits", length, data_bits.len()));
}

fn bits_to_bytes(bits: Vec<u8>) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(bits.len() / 8);
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &b) in chunk.iter().enumerate() {
            if b == 1 {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbaImage, DynamicImage};

    #[test]
    fn test_round_trip() {
        // Create a 10x10 image (100 pixels = 300 bits capacity = ~37 bytes)
        let buffer = RgbaImage::new(10, 10);
        let mut img = DynamicImage::ImageRgba8(buffer);

        let payload = b"Hello!"; // 6 bytes

        embed_bytes(&mut img, payload).expect("Failed to embed");
        let extracted = extract_bytes(&img).expect("Failed to extract");

        assert_eq!(payload.to_vec(), extracted);
    }

    #[test]
    fn test_capacity_check() {
        let buffer = RgbaImage::new(2, 2); // 4 pixels = 12 bits capacity.
        // Need 32 bits for header. So this should fail.
        let mut img = DynamicImage::ImageRgba8(buffer);
        let payload = b"A";

        assert!(embed_bytes(&mut img, payload).is_err());
    }

    #[test]
    fn test_large_payload() {
        // 100x100 = 10000 pixels = 30000 bits = 3750 bytes
        let buffer = RgbaImage::new(100, 100);
        let mut img = DynamicImage::ImageRgba8(buffer);

        let payload = vec![0xAA; 1000];
        embed_bytes(&mut img, &payload).expect("Failed to embed");
        let extracted = extract_bytes(&img).expect("Failed to extract");

        assert_eq!(payload, extracted);
    }
}
