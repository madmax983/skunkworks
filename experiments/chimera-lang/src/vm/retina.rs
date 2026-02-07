#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "nova")]
pub const RETINA_WIDTH: usize = 64;
#[cfg(feature = "nova")]
pub const RETINA_HEIGHT: usize = 32;

#[cfg(feature = "nova")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Retina {
    pub width: usize,
    pub height: usize,
    /// Buffer stores (character, (r, g, b))
    pub buffer: Vec<Vec<(char, (u8, u8, u8))>>,
}

#[cfg(feature = "nova")]
impl Default for Retina {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "nova")]
impl Retina {
    pub fn new() -> Self {
        // Initialize with spaces and black background (effectively empty)
        // We only store FG color for now in tuple.
        // Let's assume (char, fg_color).
        // Default color white?
        let buffer = vec![vec![(' ', (255, 255, 255)); RETINA_WIDTH]; RETINA_HEIGHT];
        Self {
            width: RETINA_WIDTH,
            height: RETINA_HEIGHT,
            buffer,
        }
    }

    pub fn draw(&mut self, y: usize, x: usize, ch: char, r: u8, g: u8, b: u8) {
        if y < self.height && x < self.width {
            self.buffer[y][x] = (ch, (r, g, b));
        }
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = (' ', (r, g, b));
            }
        }
    }
}
