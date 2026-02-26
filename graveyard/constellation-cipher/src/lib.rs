use anyhow::Result;
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct Star {
    pub x: u32,
    pub y: u32,
    pub byte: u8,
    pub color: Rgba<u8>,
}

pub fn derive_seed(key: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.finalize().into()
}

fn byte_to_color(byte: u8) -> Rgba<u8> {
    // Map byte to a visible color.
    // Using R=byte, G=255-byte, B=128, A=255
    Rgba([byte, 255u8.wrapping_sub(byte), 128, 255])
}

fn color_to_byte(color: Rgba<u8>) -> u8 {
    // Recover byte from Red channel
    color[0]
}

fn find_free_spot(
    rng: &mut ChaCha20Rng,
    width: u32,
    height: u32,
    occupied: &mut [bool],
) -> Result<(u32, u32)> {
    let max_attempts = 1000;
    for _ in 0..max_attempts {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = (y * width + x) as usize;
        if !occupied[idx] {
            occupied[idx] = true;
            return Ok((x, y));
        }
    }
    anyhow::bail!("Failed to find free spot. Image too small for payload density.")
}

pub fn encode(data: &[u8], key: &str, width: u32, height: u32) -> Result<RgbaImage> {
    let seed = derive_seed(key);
    let mut rng = ChaCha20Rng::from_seed(seed);

    let mut img: RgbaImage = ImageBuffer::new(width, height);
    // Initialize black
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255]);
    }

    let mut occupied = vec![false; (width * height) as usize];

    // Header: Length (4 bytes, Big Endian)
    let len_bytes = (data.len() as u32).to_be_bytes();

    // Combine
    let total_bytes = len_bytes.len() + data.len();
    if total_bytes as u32 > width * height {
        anyhow::bail!("Payload too large for image dimensions.");
    }

    // Encode Length
    for &b in &len_bytes {
        let (x, y) = find_free_spot(&mut rng, width, height, &mut occupied)?;
        img.put_pixel(x, y, byte_to_color(b));
    }

    // Encode Data
    for &b in data {
        let (x, y) = find_free_spot(&mut rng, width, height, &mut occupied)?;
        img.put_pixel(x, y, byte_to_color(b));
    }

    // Add Decoys (Noise)
    // Fill 1% of remaining space with noise stars
    // Ensure they don't overwrite existing data (occupied check)
    // But they must be marked occupied to prevent... wait.
    // Decoys are added LAST. They don't affect payload placement.
    // So we just iterate and place.
    let noise_count = (width * height) / 100;
    for _ in 0..noise_count {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = (y * width + x) as usize;
        if !occupied[idx] {
            // Random color
            img.put_pixel(x, y, Rgba([rng.gen(), rng.gen(), rng.gen(), 255]));
            occupied[idx] = true;
        }
    }

    Ok(img)
}

pub fn decode(img: &RgbaImage, key: &str) -> Result<Vec<u8>> {
    let seed = derive_seed(key);
    let mut rng = ChaCha20Rng::from_seed(seed);
    let (width, height) = img.dimensions();
    let mut occupied = vec![false; (width * height) as usize];

    // Read Length (4 bytes)
    let mut len_bytes = [0u8; 4];
    for i in 0..4 {
        let (x, y) = find_free_spot(&mut rng, width, height, &mut occupied)?;
        let pixel = img.get_pixel(x, y);
        len_bytes[i] = color_to_byte(*pixel);
    }
    let len = u32::from_be_bytes(len_bytes) as usize;

    if len > (width * height) as usize {
        anyhow::bail!("Decoded length exceeds image capacity. Invalid key or corrupted image.");
    }

    // Read Data
    let mut data = Vec::with_capacity(len);
    for _ in 0..len {
        let (x, y) = find_free_spot(&mut rng, width, height, &mut occupied)?;
        let pixel = img.get_pixel(x, y);
        data.push(color_to_byte(*pixel));
    }

    Ok(data)
}

pub fn recover_stars(img: &RgbaImage, key: &str) -> Result<Vec<Star>> {
    let seed = derive_seed(key);
    let mut rng = ChaCha20Rng::from_seed(seed);
    let (width, height) = img.dimensions();
    let mut occupied = vec![false; (width * height) as usize];

    let mut stars = Vec::new();

    // Read Length
    let mut len_bytes = [0u8; 4];
    for i in 0..4 {
        let (x, y) = find_free_spot(&mut rng, width, height, &mut occupied)?;
        let pixel = img.get_pixel(x, y);
        let b = color_to_byte(*pixel);
        len_bytes[i] = b;
        stars.push(Star {
            x,
            y,
            byte: b,
            color: *pixel,
        });
    }
    let len = u32::from_be_bytes(len_bytes) as usize;

    // Read Data
    for _ in 0..len {
        let (x, y) = find_free_spot(&mut rng, width, height, &mut occupied)?;
        let pixel = img.get_pixel(x, y);
        let b = color_to_byte(*pixel);
        stars.push(Star {
            x,
            y,
            byte: b,
            color: *pixel,
        });
    }

    Ok(stars)
}
