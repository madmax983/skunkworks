use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[allow(dead_code)]
pub struct Platter {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub capacity: usize,
}

impl Platter {
    pub fn new(width: usize, height: usize) -> Self {
        let capacity = width * height; // In bytes, mapped to cells
        Self {
            data: vec![0; capacity],
            width,
            height,
            capacity,
        }
    }

    pub fn load_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut cursor = 0;
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                // Skip hidden files and target directory to avoid massive binary blobs
                if entry.path().to_string_lossy().contains("/.")
                    || entry.path().to_string_lossy().contains("/target")
                {
                    continue;
                }

                if let Ok(content) = fs::read(entry.path()) {
                    for byte in content {
                        if cursor < self.capacity {
                            self.data[cursor] = byte;
                            cursor += 1;
                        } else {
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn flip_bit(&mut self, byte_index: usize, bit_index: u8) {
        if byte_index < self.data.len() {
            self.data[byte_index] ^= 1 << bit_index;
        }
    }

    #[allow(dead_code)]
    pub fn get_byte(&self, index: usize) -> u8 {
        if index < self.data.len() {
            self.data[index]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_bit() {
        let mut platter = Platter::new(10, 10);
        // Initially 0
        assert_eq!(platter.get_byte(0), 0);

        // Flip bit 0 of byte 0
        platter.flip_bit(0, 0);
        assert_eq!(platter.get_byte(0), 1);

        // Flip bit 1 of byte 0
        platter.flip_bit(0, 1);
        assert_eq!(platter.get_byte(0), 3); // 1 | 2 = 3

        // Flip bit 0 again (toggle off)
        platter.flip_bit(0, 0);
        assert_eq!(platter.get_byte(0), 2);
    }
}
