use anyhow::{anyhow, Result};
use image::{DynamicImage};

pub fn encode(img: &DynamicImage, data: &[u8]) -> Result<DynamicImage> {
    let mut rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let capacity_bits = (width as usize) * (height as usize) * 3;
    let needed_bits = 32 + data.len() * 8;

    if needed_bits > capacity_bits {
        return Err(anyhow!("Image too small to hold data. Capacity: {} bits, Needed: {} bits", capacity_bits, needed_bits));
    }

    let len_bytes = (data.len() as u32).to_le_bytes();
    let mut bits = Vec::with_capacity(needed_bits);

    for b in len_bytes.iter().chain(data.iter()) {
        for i in 0..8 {
            bits.push((b >> i) & 1);
        }
    }

    let mut bit_idx = 0;
    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = rgb_img.get_pixel_mut(x, y);
            for c in 0..3 {
                if bit_idx >= bits.len() {
                    break 'outer;
                }
                let bit = bits[bit_idx];
                pixel[c] = (pixel[c] & !1) | bit;
                bit_idx += 1;
            }
        }
    }

    Ok(DynamicImage::ImageRgb8(rgb_img))
}

pub fn decode(img: &DynamicImage) -> Result<Vec<u8>> {
    let rgb_img = img.to_rgb8();

    // We need to iterate exactly as we encoded
    let mut channel_values = rgb_img.pixels().flat_map(|p| p.0);

    // 1. Read Length (32 bits)
    let mut len: u32 = 0;
    for i in 0..32 {
        if let Some(val) = channel_values.next() {
            let bit = val & 1;
            if bit == 1 {
                len |= 1 << i;
            }
        } else {
            return Err(anyhow!("Image too small or corrupt header"));
        }
    }

    // 2. Read Data
    let mut data = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let mut byte = 0u8;
        for i in 0..8 {
             if let Some(val) = channel_values.next() {
                let bit = val & 1;
                if bit == 1 {
                    byte |= 1 << i;
                }
             } else {
                 return Err(anyhow!("Unexpected end of image data"));
             }
        }
        data.push(byte);
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbImage, Rgb};

    #[test]
    fn test_roundtrip() {
        let mut img = RgbImage::new(100, 100);
        for p in img.pixels_mut() { *p = Rgb([100, 100, 100]); }
        let dynamic = DynamicImage::ImageRgb8(img);

        let data = vec![1, 2, 3, 4, 5, 255, 0];
        let encoded = encode(&dynamic, &data).unwrap();
        let decoded = decode(&encoded).unwrap();

        assert_eq!(data, decoded);
    }
}
