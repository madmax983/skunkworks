use anyhow::{anyhow, Result};
use image::{DynamicImage, GenericImageView, GenericImage, Rgba};

pub fn encode(image: &mut DynamicImage, payload: &[u8]) -> Result<()> {
    let len = payload.len() as u32;
    let mut data = Vec::with_capacity(4 + payload.len());
    data.extend_from_slice(&len.to_le_bytes());
    data.extend_from_slice(payload);

    let (width, height) = image.dimensions();
    let capacity_bits = (width as usize * height as usize * 3 * 2); // 3 channels * 2 bits
    let required_bits = data.len() * 8;

    if required_bits > capacity_bits {
        return Err(anyhow!("Image too small for payload. Capacity: {} bits, Required: {} bits", capacity_bits, required_bits));
    }

    let mut bit_idx = 0;

    'outer: for y in 0..height {
        for x in 0..width {
            if bit_idx >= required_bits {
                break 'outer;
            }

            let pixel = image.get_pixel(x, y);
            let mut rgba = pixel.0; // [u8; 4]

            // Modify R, G, B channels (indices 0, 1, 2)
            for c in 0..3 {
                if bit_idx >= required_bits {
                    break;
                }

                // Get 2 bits from data
                let byte_idx = bit_idx / 8;
                let bit_offset = bit_idx % 8;
                let bits = (data[byte_idx] >> bit_offset) & 0x03;

                // Clear last 2 bits of channel and set new bits
                rgba[c] = (rgba[c] & 0xFC) | bits;

                bit_idx += 2;
            }

            image.put_pixel(x, y, Rgba(rgba));
        }
    }

    Ok(())
}

pub fn decode(image: &DynamicImage) -> Result<Vec<u8>> {
    let (width, height) = image.dimensions();

    let mut current_byte = 0u8;
    let mut bits_collected = 0;
    let mut bytes = Vec::new();

    // We expect at least 4 bytes (header)
    let mut expected_len = 4;
    let mut header_read = false;

    'outer: for y in 0..height {
        for x in 0..width {
             let pixel = image.get_pixel(x, y);
             let rgba = pixel.0;

             for c in 0..3 {
                 // Stop if we have read everything
                 if header_read && bytes.len() >= expected_len {
                     break 'outer;
                 }

                 // If we haven't read header yet, keep going until bytes.len() == 4

                 let bits = rgba[c] & 0x03;
                 current_byte |= bits << bits_collected;
                 bits_collected += 2;

                 if bits_collected >= 8 {
                     bytes.push(current_byte);
                     current_byte = 0;
                     bits_collected = 0;

                     if !header_read && bytes.len() == 4 {
                         let mut len_bytes = [0u8; 4];
                         len_bytes.copy_from_slice(&bytes[0..4]);
                         let payload_len = u32::from_le_bytes(len_bytes);
                         expected_len = 4 + payload_len as usize;
                         header_read = true;
                     }
                 }
             }
        }
    }

    if bytes.len() < 4 {
         return Err(anyhow!("No valid header found (image too small or corrupted)."));
    }

    if bytes.len() < expected_len {
         return Err(anyhow!("Incomplete payload. Expected {} bytes, got {}.", expected_len, bytes.len()));
    }

    // Return only the payload, exclude header
    Ok(bytes[4..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbaImage, DynamicImage};

    #[test]
    fn test_encode_decode() {
        let mut img = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        let payload = b"Hello, World! This is a test.";

        encode(&mut img, payload).unwrap();
        let decoded = decode(&img).unwrap();

        assert_eq!(payload.to_vec(), decoded);
    }

    #[test]
    fn test_large_payload() {
        let mut img = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
        // Capacity: 100*100*3*2 = 60,000 bits = 7,500 bytes.
        let payload = vec![0xAB; 7000];

        encode(&mut img, &payload).unwrap();
        let decoded = decode(&img).unwrap();

        assert_eq!(payload, decoded);
    }

    #[test]
    fn test_capacity_check() {
        let mut img = DynamicImage::ImageRgba8(RgbaImage::new(1, 1)); // 1 pixel = 3 channels * 2 bits = 6 bits
        let payload = b"A"; // 8 bits + 32 header = 40 bits

        let result = encode(&mut img, payload);
        assert!(result.is_err());
    }
}
