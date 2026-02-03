use image::{Rgb, RgbImage};
use rand::Rng;

/// Generates a procedural "plasma" image to use as a cover.
pub fn generate_plasma(width: u32, height: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let mut rng = rand::thread_rng();

    // Random offsets for the plasma to make it unique each time
    let offset_x: f64 = rng.gen_range(0.0..100.0);
    let offset_y: f64 = rng.gen_range(0.0..100.0);
    let scale = 0.05;

    for y in 0..height {
        for x in 0..width {
            let cx = x as f64 * scale + offset_x;
            let cy = y as f64 * scale + offset_y;

            // Plasma function: combination of sines
            let v1 = cx.sin();
            let v2 = cy.sin();
            let v3 = ((cx + cy) / 2.0).sin();
            let v4 = (((cx * cx + cy * cy).sqrt()) / 2.0).sin();

            let v = (v1 + v2 + v3 + v4) / 4.0; // Range -1 to 1

            // Map -1..1 to 0..1
            let t = (v + 1.0) / 2.0;

            // Create a psychedelic palette
            let r = (t * 255.0) as u8;
            let g = ((t * 2.0 * std::f64::consts::PI).sin().abs() * 255.0) as u8;
            let b = ((1.0 - t) * 255.0) as u8;

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    img
}

/// Embeds data into the LSB of the image channels.
///
/// Format:
/// [Header: u32 (32 bits) - Little Endian Length of payload bytes]
/// [Payload: N bytes]
pub fn embed(image: &mut RgbImage, data: &[u8]) -> anyhow::Result<()> {
    let capacity_bits = (image.width() * image.height() * 3) as usize;
    let required_bits = 32 + data.len() * 8;

    if required_bits > capacity_bits {
        return Err(anyhow::anyhow!(
            "Image too small to hold data. Capacity: {} bits, Required: {} bits",
            capacity_bits,
            required_bits
        ));
    }

    let mut bit_iter = DataBitIterator::new(data);

    // Iterate over pixels and channels
    // Note: We traverse row by row
    for pixel in image.pixels_mut() {
        for channel in 0..3 {
            if let Some(bit) = bit_iter.next() {
                // Clear LSB and set new bit
                pixel[channel] = (pixel[channel] & 0xFE) | bit;
            } else {
                return Ok(()); // All bits written
            }
        }
    }

    Ok(())
}

/// Extracts data from the LSB of the image channels.
pub fn extract(image: &RgbImage) -> anyhow::Result<Vec<u8>> {
    let mut bits = Vec::new();
    let mut extracted_bytes = Vec::new();
    let mut length: Option<u32> = None;

    // Iterate over pixels and channels
    for pixel in image.pixels() {
        for channel in 0..3 {
            let bit = pixel[channel] & 1;
            bits.push(bit);

            // Check if we have enough for length (32 bits)
            if length.is_none() {
                if bits.len() == 32 {
                    // Reconstruct u32 length (Little Endian)
                    let mut len: u32 = 0;
                    for (i, &b) in bits.iter().enumerate() {
                        if b == 1 {
                            len |= 1 << i;
                        }
                    }
                    length = Some(len);
                    bits.clear(); // Reset for payload
                }
            } else {
                // We are reading payload
                if bits.len() == 8 {
                    let mut byte: u8 = 0;
                    for (i, &b) in bits.iter().enumerate() {
                        if b == 1 {
                            byte |= 1 << i;
                        }
                    }
                    extracted_bytes.push(byte);
                    bits.clear();

                    if extracted_bytes.len() as u32 == length.unwrap() {
                        return Ok(extracted_bytes);
                    }
                }
            }
        }
    }

    if let Some(len) = length {
        if extracted_bytes.len() as u32 != len {
            return Err(anyhow::anyhow!(
                "Incomplete data. Expected {} bytes, got {}",
                len,
                extracted_bytes.len()
            ));
        }
        Ok(extracted_bytes)
    } else {
        Err(anyhow::anyhow!("Could not extract length header"))
    }
}

struct DataBitIterator<'a> {
    data: &'a [u8],
    byte_index: usize,
    bit_index: usize,
    length_emitted: bool,
    length_bits: Vec<u8>,
    length_bit_index: usize,
}

impl<'a> DataBitIterator<'a> {
    fn new(data: &'a [u8]) -> Self {
        let len = data.len() as u32;
        let mut length_bits = Vec::new();
        for i in 0..32 {
            length_bits.push(((len >> i) & 1) as u8);
        }

        Self {
            data,
            byte_index: 0,
            bit_index: 0,
            length_emitted: false,
            length_bits,
            length_bit_index: 0,
        }
    }
}

impl<'a> Iterator for DataBitIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Emit length header first
        if !self.length_emitted {
            if self.length_bit_index < 32 {
                let bit = self.length_bits[self.length_bit_index];
                self.length_bit_index += 1;
                return Some(bit);
            } else {
                self.length_emitted = true;
            }
        }

        // Emit data
        if self.byte_index < self.data.len() {
            let bit = (self.data[self.byte_index] >> self.bit_index) & 1;
            self.bit_index += 1;
            if self.bit_index == 8 {
                self.bit_index = 0;
                self.byte_index += 1;
            }
            return Some(bit);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steganography() {
        let w = 100;
        let h = 100;
        let mut img = generate_plasma(w, h);
        let payload = "Hello, World! This is a secret message.";

        embed(&mut img, payload.as_bytes()).expect("Failed to embed");

        let extracted = extract(&img).expect("Failed to extract");
        let extracted_str = String::from_utf8(extracted).expect("Invalid UTF8");

        assert_eq!(payload, extracted_str);
    }

    #[test]
    fn test_capacity_check() {
        let w = 10;
        let h = 10;
        // Capacity = 10*10*3 = 300 bits.
        // Header = 32 bits.
        // Remaining = 268 bits = 33 bytes.
        let mut img = generate_plasma(w, h);
        let payload = vec![0u8; 40]; // 40 bytes > 33 bytes

        let result = embed(&mut img, &payload);
        assert!(result.is_err());
    }
}
