use anyhow::{anyhow, Result};
use image::{DynamicImage, GenericImage, GenericImageView};

pub fn embed(mut cover: DynamicImage, data: &[u8]) -> Result<DynamicImage> {
    let len = data.len() as u32;
    // Header is 4 bytes (u32)
    let total_bits = (4 + data.len()) as u64 * 8;
    // 3 channels * 2 bits = 6 bits per pixel
    let pixels_needed = (total_bits + 5) / 6;

    let (width, height) = cover.dimensions();
    if pixels_needed > (width as u64 * height as u64) {
        return Err(anyhow!(
            "Image too small to hold data. Needed {} pixels, got {}",
            pixels_needed,
            width as u64 * height as u64
        ));
    }

    let mut byte_iter = len
        .to_le_bytes()
        .to_vec()
        .into_iter()
        .chain(data.iter().cloned());
    let mut current_byte = byte_iter.next();
    let mut bit_cursor = 0; // 0..8

    for y in 0..height {
        for x in 0..width {
            if current_byte.is_none() {
                break;
            }

            let mut pixel = cover.get_pixel(x, y);

            // R, G, B channels
            for channel_idx in 0..3 {
                if current_byte.is_none() {
                    break;
                }

                let byte_val = current_byte.unwrap();
                // Extract 2 bits starting at bit_cursor
                let bits = (byte_val >> bit_cursor) & 0x03;

                // Embed into channel (clear last 2 bits, then OR with data)
                pixel[channel_idx] = (pixel[channel_idx] & 0xFC) | bits;

                bit_cursor += 2;
                if bit_cursor >= 8 {
                    bit_cursor = 0;
                    current_byte = byte_iter.next();
                }
            }
            cover.put_pixel(x, y, pixel);
        }
    }

    Ok(cover)
}

pub fn extract(image: &DynamicImage) -> Result<Vec<u8>> {
    let (width, height) = image.dimensions();

    let mut bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_cursor = 0;

    let mut len: Option<u32> = None;
    let mut header_bytes = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            for channel_idx in 0..3 {
                let val = pixel[channel_idx];
                let bits = val & 0x03;

                current_byte |= bits << bit_cursor;
                bit_cursor += 2;

                if bit_cursor >= 8 {
                    if len.is_none() {
                        header_bytes.push(current_byte);
                        if header_bytes.len() == 4 {
                            len = Some(u32::from_le_bytes([
                                header_bytes[0],
                                header_bytes[1],
                                header_bytes[2],
                                header_bytes[3],
                            ]));
                        }
                    } else {
                        bytes.push(current_byte);
                        if let Some(l) = len {
                            if bytes.len() as u32 == l {
                                return Ok(bytes);
                            }
                        }
                    }
                    current_byte = 0;
                    bit_cursor = 0;
                }
            }
            if let Some(l) = len {
                if bytes.len() as u32 == l {
                    return Ok(bytes);
                }
            }
        }
    }

    if let Some(l) = len {
        return Err(anyhow!(
            "Incomplete data. Expected {} bytes, got {}",
            l,
            bytes.len()
        ));
    }

    Err(anyhow!(
        "Failed to read header (image might be empty or too small)"
    ))
}
