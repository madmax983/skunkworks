//! The Visual Buffer of the Chimera Organism.
//!
//! The `Retina` module provides a dedicated 2D terminal-like display buffer
//! (64x32 by default) that a [`ChimeraVM`] can draw to using specific enzymes.
//! It allows organisms to visualize their internal state, output data, or
//! construct crude user interfaces entirely from DNA execution.
//!
//! Organisms manipulate this buffer by pushing RGB colors, characters, and coordinates
//! onto the stack, then executing `retina_draw` or `rasterize`.

#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "nova")]
/// The default width of the Retina buffer in cells.
pub const RETINA_WIDTH: usize = 64;
#[cfg(feature = "nova")]
/// The default height of the Retina buffer in cells.
pub const RETINA_HEIGHT: usize = 32;

#[cfg(feature = "nova")]
/// A 2D visual buffer for rendering text and colors.
///
/// The `Retina` is attached to a [`ChimeraVM`] and acts as a specialized output device.
/// It maintains a grid of characters, each with an associated foreground RGB color.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Retina {
    /// The horizontal dimension of the buffer.
    pub width: usize,
    /// The vertical dimension of the buffer.
    pub height: usize,
    /// The underlying 2D grid storing tuples of `(character, (red, green, blue))`.
    #[allow(clippy::type_complexity)]
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
    /// Creates a new, blank Retina initialized with space characters and white text.
    ///
    /// # Examples
    ///
    /// ```
    /// use chimera_lang::Retina;
    ///
    /// let retina = Retina::new();
    /// assert_eq!(retina.width, 64);
    /// assert_eq!(retina.height, 32);
    /// // The top-left cell is a blank space, colored white.
    /// assert_eq!(retina.buffer[0][0], (' ', (255, 255, 255)));
    /// ```
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

    /// Draws a character with a specific RGB color at the given coordinates.
    ///
    /// Silently ignores out-of-bounds coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use chimera_lang::Retina;
    ///
    /// let mut retina = Retina::new();
    /// // Draw a red 'X' at (y: 5, x: 10)
    /// retina.draw(5, 10, 'X', 255, 0, 0);
    /// assert_eq!(retina.buffer[5][10], ('X', (255, 0, 0)));
    ///
    /// // Drawing out of bounds does nothing, preventing panics
    /// retina.draw(100, 100, '?', 0, 0, 0);
    /// ```
    pub fn draw(&mut self, y: usize, x: usize, ch: char, r: u8, g: u8, b: u8) {
        if y < self.height && x < self.width {
            self.buffer[y][x] = (ch, (r, g, b));
        }
    }

    /// Clears the entire buffer, setting all cells to a space character with the provided RGB color.
    ///
    /// # Examples
    ///
    /// ```
    /// use chimera_lang::Retina;
    ///
    /// let mut retina = Retina::new();
    /// // Clear screen to black
    /// retina.clear(0, 0, 0);
    /// assert_eq!(retina.buffer[15][15], (' ', (0, 0, 0)));
    /// ```
    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = (' ', (r, g, b));
            }
        }
    }
}

// --- VM Execution Logic ---

#[cfg(feature = "nova")]
/// Pops X, Y, Character, and Color from the stack to draw on the VM's Retina.
///
/// Modifies the `Retina` buffer attached to the VM. Costs 1 Energy.
/// The `color` value is an integer interpreted as a 24-bit RGB code (e.g., `0xFF0000` is Red).
///
/// Returns `None` as it does not perform a jump.
pub fn exec_retina_draw(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 4 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let char_val = vm.stack.pop().unwrap();
        let color_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(c), Value::Int(rgb)) =
            (x_val, y_val, char_val, color_val)
        {
            if x < 0 || y < 0 {
                vm.output.push("Error: Negative coordinates".to_string());
                return None;
            }
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            let ch = (c as u8) as char;

            vm.retina.draw(y as usize, x as usize, ch, r, g, b);
            vm.energy = vm.energy.saturating_sub(1);
        } else {
            vm.output
                .push("Error: Type mismatch for retina_draw".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for retina_draw".to_string());
    }
    None
}

#[cfg(feature = "nova")]
/// Pops a Color from the stack to clear the entire Retina buffer.
///
/// The `color` value is an integer interpreted as a 24-bit RGB code.
/// This operation resets all cells to spaces with the new color. Costs 10 Energy.
///
/// Returns `None` as it does not perform a jump.
pub fn exec_retina_clear(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(rgb) = val {
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            vm.retina.clear(r, g, b);
            vm.energy = vm.energy.saturating_sub(10);
        } else {
            vm.output
                .push("Error: Type mismatch for retina_clear".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for retina_clear".to_string());
    }
    None
}

#[cfg(feature = "nova")]
/// Pushes the width and height of the Retina buffer onto the stack.
///
/// Order on stack: `Width`, then `Height`.
///
/// Returns `None` as it does not perform a jump.
pub fn exec_retina_size(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.stack.push(Value::Int(vm.retina.width as i64));
    vm.stack.push(Value::Int(vm.retina.height as i64));
    None
}

#[cfg(feature = "nova")]
/// Pops a Y coordinate and pushes a Junction array of all pixels in that row.
///
/// Returns a Junction of integers representing the RGB colors of each pixel in the specified row.
/// If out-of-bounds, pushes an empty Junction array and writes to `vm.output`.
///
/// Returns `None` as it does not perform a jump.
pub fn exec_scanline(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(y) = val {
            if y >= 0 && (y as usize) < vm.retina.height {
                let row = &vm.retina.buffer[y as usize];
                let mut pixels = Vec::new();
                for (_, (r, g, b)) in row {
                    let rgb = ((*r as i64) << 16) | ((*g as i64) << 8) | (*b as i64);
                    pixels.push(Value::Int(rgb));
                }
                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, pixels));
            } else {
                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, vec![]));
                vm.output
                    .push("Error: Scanline index out of bounds".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for scanline".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for scanline".to_string());
    }
    None
}

#[cfg(feature = "nova")]
/// Renders an array of pixels directly to the Retina buffer.
///
/// Pops Mode, Data (Junction or Int), X, and Y from the stack.
/// Draws a horizontal line of pixels starting at `(x, y)`.
///
/// Mode behaviors:
/// - `1`: Scatter (adds a random -2..=2 offset to X for each pixel)
/// - `2`: XOR (XORs the new pixel RGB against the existing pixel RGB)
/// - `3`: Sort (Sorts the given Data array by brightness before drawing)
/// - `other`: Default (direct replacement drawing)
///
/// The cost depends on the number of pixels in the data array (`len / 10` Energy).
///
/// Returns `None` as it does not perform a jump.
pub fn exec_rasterize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 4 {
        let mode_val = vm.stack.pop().unwrap();
        let data_val = vm.stack.pop().unwrap();
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x), Value::Int(mode)) = (y_val, x_val, mode_val) {
            if x < 0 || y < 0 {
                vm.output.push("Error: Negative coordinates".to_string());
                return None;
            }
            let mut pixel_data = Vec::new();

            match data_val {
                Value::Junction(_, vals) => {
                    for v in vals {
                        if let Value::Int(i) = v {
                            pixel_data.push(i);
                        }
                    }
                }
                Value::Int(i) => pixel_data.push(i),
                _ => {}
            }

            if mode == 3 {
                // Sort by brightness?
                // Approximate brightness: sum of R+G+B
                pixel_data.sort_by_key(|rgb| {
                    let r = (rgb >> 16) & 0xFF;
                    let g = (rgb >> 8) & 0xFF;
                    let b = rgb & 0xFF;
                    r + g + b
                });
            }

            let start_x = x as usize;
            let start_y = y as usize;

            for (i, &rgb) in pixel_data.iter().enumerate() {
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;

                let mut draw_x = start_x + i;
                let draw_y = start_y;

                if mode == 1 {
                    // Scatter
                    let mut rng = rand::thread_rng();
                    use rand::Rng;
                    let offset = rng.gen_range(-2..=2);
                    // Use isize for calculation to handle negative result before max
                    draw_x = (draw_x as isize + offset).max(0) as usize;
                }

                if draw_y < vm.retina.height && draw_x < vm.retina.width {
                    if mode == 2 {
                        // XOR
                        let (_, (or, og, ob)) = vm.retina.buffer[draw_y][draw_x];
                        let nr = r ^ or;
                        let ng = g ^ og;
                        let nb = b ^ ob;
                        vm.retina.buffer[draw_y][draw_x] = ('#', (nr, ng, nb));
                    } else {
                        // Default
                        vm.retina.buffer[draw_y][draw_x] = ('#', (r, g, b));
                    }
                }
            }
            vm.energy = vm.energy.saturating_sub(pixel_data.len() as i64 / 10);
        } else {
            vm.output
                .push("Error: Type mismatch for rasterize".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for rasterize".to_string());
    }
    None
}
