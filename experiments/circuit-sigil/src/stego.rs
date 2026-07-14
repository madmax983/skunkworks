use anyhow::{Context, Result};
use image::RgbaImage;

pub fn embed(img: &mut RgbaImage, data: &str, pads: &[(u32, u32)]) -> Result<()> {
    let payload = data.as_bytes();
    let len = payload.len() as u32;
    let mut bits: Vec<u8> = Vec::new();

    // Push length (32 bits, Big Endian)
    for i in (0..32).rev() {
        bits.push(((len >> i) & 1) as u8);
    }

    // Push data
    for &byte in payload {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1);
        }
    }

    let mut bit_idx = 0;
    let radius = 6;
    let hole_radius = 2;

    // Iterate pads
    for &(cx, cy) in pads {
        // Iterate pixels in annulus (scanline order for determinism)
        for y in (cy as i32 - radius)..(cy as i32 + radius + 1) {
            for x in (cx as i32 - radius)..(cx as i32 + radius + 1) {
                if bit_idx >= bits.len() {
                    return Ok(());
                }

                if x < 0 || x >= img.width() as i32 || y < 0 || y >= img.height() as i32 {
                    continue;
                }

                let dx = x - cx as i32;
                let dy = y - cy as i32;
                let dist_sq = dx * dx + dy * dy;

                // Check annulus: hole < dist <= radius
                if dist_sq > hole_radius * hole_radius && dist_sq <= radius * radius {
                    let pixel = img.get_pixel_mut(x as u32, y as u32);
                    // Embed in R, G, B
                    for c in 0..3 {
                        if bit_idx < bits.len() {
                            let val = pixel[c];
                            let bit = bits[bit_idx];
                            // Clear LSB and set new bit
                            pixel[c] = (val & !1) | bit;
                            bit_idx += 1;
                        }
                    }
                }
            }
        }
    }

    if bit_idx < bits.len() {
        return Err(anyhow::anyhow!("Not enough capacity in image for data."));
    }

    Ok(())
}

pub fn extract(img: &RgbaImage, pads: &[(u32, u32)]) -> Result<String> {
    let mut bits: Vec<u8> = Vec::new();
    let radius = 6;
    let hole_radius = 2;

    for &(cx, cy) in pads {
        for y in (cy as i32 - radius)..(cy as i32 + radius + 1) {
            for x in (cx as i32 - radius)..(cx as i32 + radius + 1) {
                if x < 0 || x >= img.width() as i32 || y < 0 || y >= img.height() as i32 {
                    continue;
                }

                let dx = x - cx as i32;
                let dy = y - cy as i32;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq > hole_radius * hole_radius && dist_sq <= radius * radius {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    for c in 0..3 {
                        bits.push(pixel[c] & 1);
                    }
                }
            }
        }
    }

    if bits.len() < 32 {
        return Err(anyhow::anyhow!("No data found (header incomplete)."));
    }

    // Parse Length
    let mut len: u32 = 0;
    for &bit in bits.iter().take(32) {
        len = (len << 1) | bit as u32;
    }

    let needed_bits = 32 + (len as usize * 8);
    if bits.len() < needed_bits {
        return Err(anyhow::anyhow!(
            "Data truncated. Expected {} bits, got {}.",
            needed_bits,
            bits.len()
        ));
    }

    let mut data_bytes = Vec::new();
    for i in 0..len as usize {
        let mut byte = 0u8;
        for b in 0..8 {
            let bit_pos = 32 + i * 8 + b;
            byte = (byte << 1) | bits[bit_pos];
        }
        data_bytes.push(byte);
    }

    String::from_utf8(data_bytes).context("Failed to decode UTF-8 string")
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    #[test]
    fn test_stego_roundtrip() {
        let mut img = RgbaImage::new(100, 100);
        let pads = vec![(50, 50)];
        let data = "Hello, World!";

        // Initialize with random noise to simulate existing image
        for p in img.pixels_mut() {
            *p = Rgba([100, 100, 100, 255]);
        }

        embed(&mut img, data, &pads).unwrap();
        let result = extract(&img, &pads).unwrap();
        assert_eq!(result, data);
    }
}
