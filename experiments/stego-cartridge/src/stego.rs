//! LSB steganography engine for the Stego-Cartridge framework.
//!
//! This module provides the tools to generate deterministic cover images,
//! embed arbitrary byte slices (bytecode) into the Least Significant Bits (LSB)
//! of an image's pixels, and extract them back out perfectly.

use image::{Rgba, RgbaImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Embeds raw byte data into the Least Significant Bits (LSBs) of an RGBA image.
///
/// The payload is prepended with a 32-bit (little-endian) header indicating the
/// length of the data. Each bit of the header and data replaces the lowest bit
/// of the Red, Green, Blue, and Alpha channels of the image's pixels sequentially.
///
/// # Arguments
/// * `image` - A mutable reference to the `RgbaImage` to use as the carrier.
/// * `data` - The raw byte slice (e.g., bytecode) to embed.
///
/// # Errors
/// Returns an `Err(String)` if the provided image does not have enough pixel
/// capacity to store the 32-bit header plus the provided data.
///
/// # Examples
/// ```rust
/// use stego_cartridge::stego::{embed, generate_cover};
///
/// let payload = b"HELLO";
/// // A 2x2 image only has 16 bits of capacity (2 * 2 * 4 channels),
/// // which is too small for even the 32-bit header.
/// let mut too_small = generate_cover(payload, 2, 2);
/// assert!(embed(&mut too_small, payload).is_err());
///
/// // A 10x10 image provides 400 bits of capacity.
/// let mut valid_image = generate_cover(payload, 10, 10);
/// assert!(embed(&mut valid_image, payload).is_ok());
/// ```
pub fn embed(image: &mut RgbaImage, data: &[u8]) -> Result<(), String> {
    let capacity = (image.width() * image.height() * 4) as usize;
    let required_bits = 32 + (data.len() * 8);

    if required_bits > capacity {
        return Err(format!(
            "Data too large for image. Capacity: {} bits, Required: {} bits",
            capacity, required_bits
        ));
    }

    let mut bit_idx = 0;

    // Helper to get bit from value
    let get_bit = |val: u8, bit: usize| -> u8 { (val >> bit) & 1 };

    // Embed length (u32)
    let len = data.len() as u32;
    for i in 0..32 {
        let bit = ((len >> i) & 1) as u8;
        embed_bit(image, bit_idx, bit);
        bit_idx += 1;
    }

    // Embed data
    for &byte in data {
        for i in 0..8 {
            let bit = get_bit(byte, i);
            embed_bit(image, bit_idx, bit);
            bit_idx += 1;
        }
    }

    Ok(())
}

fn embed_bit(image: &mut RgbaImage, bit_idx: usize, bit: u8) {
    let width = image.width() as usize;
    let pixel_idx = bit_idx / 4;
    let channel_idx = bit_idx % 4;

    let x = (pixel_idx % width) as u32;
    let y = (pixel_idx / width) as u32;

    let pixel = image.get_pixel_mut(x, y);
    pixel[channel_idx] = (pixel[channel_idx] & !1) | bit;
}

/// Extracts embedded data from the Least Significant Bits (LSBs) of an image.
///
/// This function reads the first 32 bits from the image's pixel channels
/// to determine the expected payload length, then reconstructs the subsequent
/// bytes from the remaining LSBs.
///
/// # Arguments
/// * `image` - The `RgbaImage` containing the hidden payload.
///
/// # Errors
/// Returns an `Err(String)` if the extracted 32-bit length header claims a size
/// that exceeds the physical storage capacity of the image.
///
/// # Examples
/// ```rust
/// use stego_cartridge::stego::{embed, extract, generate_cover};
///
/// let payload = b"SECRET_CODE";
/// let mut image = generate_cover(payload, 10, 10);
///
/// // Hide the payload
/// embed(&mut image, payload).unwrap();
///
/// // Recover it perfectly
/// let recovered = extract(&image).unwrap();
/// assert_eq!(recovered, payload);
/// ```
pub fn extract(image: &RgbaImage) -> Result<Vec<u8>, String> {
    let mut bit_idx = 0;

    // Helper to extract bit
    let extract_bit = |idx: usize| -> u8 {
        let width = image.width() as usize;
        let pixel_idx = idx / 4;
        let channel_idx = idx % 4;
        let x = (pixel_idx % width) as u32;
        let y = (pixel_idx / width) as u32;
        image.get_pixel(x, y)[channel_idx] & 1
    };

    // Extract length
    let mut len: u32 = 0;
    for i in 0..32 {
        let bit = extract_bit(bit_idx);
        len |= (bit as u32) << i;
        bit_idx += 1;
    }

    // Sanity check length
    let capacity_bytes = (image.width() * image.height() * 4 / 8) - 4;
    if len > capacity_bytes {
        return Err(format!(
            "Extracted length {} exceeds capacity {}",
            len, capacity_bytes
        ));
    }

    // Extract data
    let mut data = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let mut byte: u8 = 0;
        for i in 0..8 {
            let bit = extract_bit(bit_idx);
            byte |= bit << i;
            bit_idx += 1;
        }
        data.push(byte);
    }

    Ok(data)
}

/// Generates a deterministic cover image to serve as the steganographic carrier.
///
/// To create a "Visual Cipher", the image's visual pattern is deterministically
/// seeded using the hash of the data it intends to carry. All generated pixels
/// ensure their Least Significant Bit is zeroed out to cleanly accept the embedded payload.
///
/// # Arguments
/// * `data` - The data to hash for the visual seed.
/// * `width` - The width of the generated cover image.
/// * `height` - The height of the generated cover image.
///
/// # Examples
/// ```rust
/// use stego_cartridge::stego::generate_cover;
///
/// let data1 = b"PROG_A";
/// let data2 = b"PROG_B";
///
/// let img1 = generate_cover(data1, 16, 16);
/// let img2 = generate_cover(data2, 16, 16);
///
/// // The visuals are deterministic based on the payload.
/// let img1_duplicate = generate_cover(data1, 16, 16);
/// assert_eq!(img1.into_raw(), img1_duplicate.into_raw());
/// ```
pub fn generate_cover(data: &[u8], width: u32, height: u32) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);

    // Seed RNG with data hash (simple sum for now)
    let seed: u64 = data.iter().map(|&b| b as u64).sum();
    let mut rng = StdRng::seed_from_u64(seed);

    for pixel in img.pixels_mut() {
        // Generate random high bits, but keep LSB 0 for now (will be overwritten)
        let r = rng.gen::<u8>() & !1;
        let g = rng.gen::<u8>() & !1;
        let b = rng.gen::<u8>() & !1;
        let a = 255; // Fully opaque
        *pixel = Rgba([r, g, b, a]);
    }

    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_extract() {
        let width = 10;
        let height = 10;
        let data = b"Hello World";

        let mut image = generate_cover(data, width, height);

        // Embed
        embed(&mut image, data).expect("Failed to embed");

        // Extract
        let extracted = extract(&image).expect("Failed to extract");

        assert_eq!(data.to_vec(), extracted);
    }

    #[test]
    fn test_capacity_check() {
        let width = 2;
        let height = 2;
        // Capacity = 2*2*4 = 16 bits.
        // Length header = 32 bits.
        // Should fail immediately.
        let mut image = RgbaImage::new(width, height);
        let data = b"A";

        let res = embed(&mut image, data);
        assert!(res.is_err());
    }
}
