use image::RgbaImage;
use anyhow::{Result, anyhow};

pub fn embed(img: &mut RgbaImage, data: &[u8]) -> Result<()> {
    let (w, h) = img.dimensions();
    let cx = (w / 2) as i32;
    let cy = (h / 2) as i32;

    // Header: 4 bytes (u32 length, Little Endian)
    let len_bytes = (data.len() as u32).to_le_bytes();

    let mut bits = Vec::with_capacity((data.len() + 4) * 8);

    for b in len_bytes.iter().chain(data.iter()) {
        for i in 0..8 {
            bits.push((b >> i) & 1);
        }
    }

    if bits.len() > (w * h * 3) as usize {
        return Err(anyhow!("Payload too large"));
    }

    let mut spiral = SpiralIter::new();
    let mut bit_idx = 0;

    // Limit spiral to avoid infinite loop
    let max_steps = (w.max(h) as i32 * w.max(h) as i32) * 4;
    let mut steps = 0;

    while bit_idx < bits.len() && steps < max_steps {
        if let Some((dx, dy)) = spiral.next() {
            steps += 1;
            let x = cx + dx;
            let y = cy + dy;

            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                for c in 0..3 {
                    if bit_idx < bits.len() {
                        let bit = bits[bit_idx];
                        pixel[c] = (pixel[c] & 0xFE) | bit;
                        bit_idx += 1;
                    }
                }
            }
        } else {
            break;
        }
    }

    if bit_idx < bits.len() {
        return Err(anyhow!("Failed to embed all data (image too small or spiral exhausted)"));
    }

    Ok(())
}

pub fn extract(img: &RgbaImage) -> Result<Vec<u8>> {
    let (w, h) = img.dimensions();
    let cx = (w / 2) as i32;
    let cy = (h / 2) as i32;

    let mut spiral = SpiralIter::new();
    let mut bits = Vec::new();
    let mut length: u32 = 0;
    let mut data = Vec::new();

    // State: 0 = Reading Length (32 bits), 1 = Reading Payload
    let mut state = 0;

    let max_steps = (w.max(h) as i32 * w.max(h) as i32) * 4;
    let mut steps = 0;

    loop {
        if let Some((dx, dy)) = spiral.next() {
            steps += 1;
            if steps > max_steps { break; }

            let x = cx + dx;
            let y = cy + dy;

            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                let pixel = img.get_pixel(x as u32, y as u32);
                for c in 0..3 {
                    let bit = pixel[c] & 1;
                    bits.push(bit);

                    if state == 0 {
                        if bits.len() == 32 {
                            // Decode length
                            length = 0;
                            for i in 0..32 {
                                if bits[i] == 1 {
                                    length |= 1 << i;
                                }
                            }
                            state = 1;
                            bits.clear();
                        }
                    } else if state == 1 {
                        if bits.len() == 8 {
                            let mut byte = 0u8;
                            for i in 0..8 {
                                if bits[i] == 1 {
                                    byte |= 1 << i;
                                }
                            }
                            data.push(byte);
                            bits.clear();

                            if data.len() as u32 == length {
                                return Ok(data);
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    Err(anyhow!("Failed to extract payload (incomplete or corrupted)"))
}

struct SpiralIter {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    step_size: i32,
    step_remain: i32,
    turn_counter: i32,
}

impl SpiralIter {
    fn new() -> Self {
        Self { x: 0, y: 0, dx: 1, dy: 0, step_size: 1, step_remain: 1, turn_counter: 0 }
    }
}

impl Iterator for SpiralIter {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.x, self.y);

        self.x += self.dx;
        self.y += self.dy;
        self.step_remain -= 1;

        if self.step_remain == 0 {
            // Turn Right
            // (1,0) -> (0,1) -> (-1,0) -> (0,-1)
            // dx=1,dy=0 -> dx=0,dy=1
            // dx=0,dy=1 -> dx=-1,dy=0
            let temp = self.dx;
            self.dx = -self.dy;
            self.dy = temp;

            self.turn_counter += 1;
            if self.turn_counter == 2 {
                self.turn_counter = 0;
                self.step_size += 1;
            }
            self.step_remain = self.step_size;
        }
        Some(res)
    }
}
