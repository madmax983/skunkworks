use anyhow::{bail, Result};

/// Represents a 2D buffer of intensity values (grayscale)
pub struct StegoBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl StegoBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn generate_plasma(&mut self, time: f64) {
        for y in 0..self.height {
            for x in 0..self.width {
                let fx = x as f64;
                let fy = y as f64;

                let v1 = (fx * 0.1 + time).sin();
                let v2 = (fy * 0.1 + time).sin();
                let v3 = ((fx + fy) * 0.1 + time).sin();
                let v4 = (((fx * fx + fy * fy).sqrt()) * 0.1 + time).sin();

                let val = (v1 + v2 + v3 + v4 + 4.0) / 8.0 * 255.0;

                let idx = y * self.width + x;
                // Preserve LSB if we wanted to be persistent, but here we overwrite.
                // If we want to support "Dynamic Noise" with hidden message, we must be careful.
                // For this tool, let's say generating noise Wipes the message.
                self.data[idx] = val as u8;
            }
        }
    }

    /// Embeds a byte slice into the buffer using LSB steganography.
    /// Format: [Length: u32 (32 bits, Little Endian)] [Data: (len * 8) bits]
    pub fn embed(&mut self, payload: &[u8]) -> Result<()> {
        let len = payload.len() as u32;
        let bits_needed = 32 + (len as usize * 8);

        if bits_needed > self.data.len() {
            bail!(
                "Payload too large for buffer: needed {} pixels, have {}",
                bits_needed,
                self.data.len()
            );
        }

        // 1. Embed Length (32 bits)
        for i in 0..32 {
            let bit = ((len >> i) & 1) as u8;
            self.data[i] = (self.data[i] & !1) | bit;
        }

        // 2. Embed Payload
        for (byte_idx, &byte) in payload.iter().enumerate() {
            for bit_idx in 0..8 {
                let bit = (byte >> bit_idx) & 1;
                let pixel_idx = 32 + (byte_idx * 8) + bit_idx;
                self.data[pixel_idx] = (self.data[pixel_idx] & !1) | bit;
            }
        }

        Ok(())
    }

    /// Extracts data from the buffer.
    pub fn extract(&self) -> Result<Vec<u8>> {
        if self.data.len() < 32 {
            bail!("Buffer too small to contain length header");
        }

        // 1. Extract Length
        let mut len: u32 = 0;
        for i in 0..32 {
            let bit = self.data[i] & 1;
            len |= (bit as u32) << i;
        }

        let bits_needed = 32 + (len as usize * 8);
        if bits_needed > self.data.len() {
            bail!(
                "Corrupt header or insufficient buffer size: claimed length {} requires {} pixels",
                len,
                bits_needed
            );
        }

        // 2. Extract Payload
        let mut payload = Vec::with_capacity(len as usize);
        for byte_idx in 0..(len as usize) {
            let mut byte: u8 = 0;
            for bit_idx in 0..8 {
                let pixel_idx = 32 + (byte_idx * 8) + bit_idx;
                let bit = self.data[pixel_idx] & 1;
                byte |= bit << bit_idx;
            }
            payload.push(byte);
        }

        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_extract_roundtrip() {
        let mut buffer = StegoBuffer::new(100, 100);
        let secret = "Hello Genesis";

        buffer.embed(secret.as_bytes()).expect("Embed failed");
        let extracted = buffer.extract().expect("Extract failed");

        assert_eq!(secret.as_bytes(), extracted.as_slice());
    }

    #[test]
    fn test_capacity_check() {
        let mut buffer = StegoBuffer::new(2, 2); // 4 pixels
        let secret = "A"; // Needs 32 + 8 = 40 pixels
        assert!(buffer.embed(secret.as_bytes()).is_err());
    }
}
