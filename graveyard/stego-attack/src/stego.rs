use image::{Rgba, RgbaImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Embeds data into the LSBs (Least Significant Bits) of the image.
/// The first 32 bits (8 pixels * 4 channels) store the length of the data (u32, little-endian).
///
/// # Arguments
///
/// * `image` - A mutable reference to the `RgbaImage` to embed into.
/// * `data` - The raw byte payload to hide in the image.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), String> {
/// use image::RgbaImage;
/// use stego_attack::stego::embed;
///
/// let mut img = RgbaImage::new(100, 100);
/// let secret_data = b"Hello, World!";
/// embed(&mut img, secret_data)?;
/// # Ok(())
/// # }
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

/// Extracts data from the LSBs (Least Significant Bits) of the image.
///
/// Assumes the first 32 bits indicate the length of the payload, and dynamically
/// reads that many bits following the header to reconstruct the data.
///
/// # Arguments
///
/// * `image` - The `RgbaImage` to read the hidden data from.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), String> {
/// use image::RgbaImage;
/// use stego_attack::stego::{embed, extract};
///
/// let mut img = RgbaImage::new(100, 100);
/// let secret_data = b"Hello, World!";
/// embed(&mut img, secret_data)?;
///
/// let extracted = extract(&img)?;
/// assert_eq!(secret_data, extracted.as_slice());
/// # Ok(())
/// # }
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

/// Generates a cover image based on the data.
/// The visual pattern is deterministic based on the data to create a "Visual Cipher".
///
/// Ensures the initial LSBs are all set to 0 to prepare the image for data embedding.
///
/// # Arguments
///
/// * `data` - The target payload, used to seed the noise generation.
/// * `width` - The width of the returned cover image.
/// * `height` - The height of the returned cover image.
///
/// # Examples
///
/// ```
/// use stego_attack::stego::generate_cover;
///
/// let secret = b"super secret code";
/// let cover = generate_cover(secret, 50, 50);
///
/// assert_eq!(cover.width(), 50);
/// assert_eq!(cover.height(), 50);
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
