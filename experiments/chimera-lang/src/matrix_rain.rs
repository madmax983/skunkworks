//! Visual "Matrix Rain" effect renderer.
//!
//! Provides the `MatrixRain` state machine and ratatui widget for rendering
//! cascading character drops to the TUI.

use rand::Rng;
use ratatui::{buffer::Buffer, layout::Rect, style::Color};

/// Represents the internal state for the Matrix Rain visual effect.
///
/// Manages the layout and rendering of animated text drop sequences
/// across the terminal grid.
/// Represents a `MatrixRain`.
pub struct MatrixRain {
    columns: Vec<Column>,
    width: u16,
    height: u16,
}

struct Column {
    x: u16,
    y: f32,
    speed: f32,
    chars: Vec<char>,
    len: usize,
}

impl Default for MatrixRain {
    fn default() -> Self {
        Self::new()
    }
}

impl MatrixRain {
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    /// Performs the `update` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of update
    /// ```
    pub fn update(&mut self, width: u16, height: u16) {
        // Resize check
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            // Don't clear, just let them fall off or clip
        }

        let mut rng = rand::thread_rng();

        // Spawn new columns randomly
        // Density control
        if self.columns.len() < (width as usize) && rng.gen_bool(0.05) {
            let x = rng.gen_range(0..width);
            self.spawn_column(x);
        }

        // Update columns
        for col in &mut self.columns {
            col.y += col.speed;
            if rng.gen_bool(0.05) {
                // Mutate a char
                if !col.chars.is_empty() {
                    let idx = rng.gen_range(0..col.chars.len());
                    col.chars[idx] = random_char();
                }
            }
        }

        // Remove off-screen
        self.columns
            .retain(|c| (c.y as i32 - c.len as i32) < height as i32);
    }

    fn spawn_column(&mut self, x: u16) {
        let mut rng = rand::thread_rng();
        let len = rng.gen_range(5..25);
        let chars: Vec<char> = (0..len).map(|_| random_char()).collect();
        self.columns.push(Column {
            x,
            y: 0.0, // Start at top
            speed: rng.gen_range(0.3..1.0),
            chars,
            len,
        });
    }

    /// Performs the `render` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of render
    /// ```
    pub fn render(&self, buf: &mut Buffer, area: Rect) {
        for col in &self.columns {
            let head_y = col.y as i32;
            for (i, ch) in col.chars.iter().enumerate() {
                let y = head_y - i as i32;
                if y >= 0 && y < area.height as i32 {
                    let x = col.x;
                    if x < area.width {
                        // Bounds check against buffer area
                        if (area.x + x) < buf.area.width && (area.y + y as u16) < buf.area.height {
                            let cell = &mut buf[(area.x + x, area.y + y as u16)];
                            cell.set_char(*ch);

                            let color = if i == 0 {
                                Color::White
                            } else if i == 1 {
                                Color::LightGreen
                            } else {
                                Color::DarkGray
                            };
                            cell.set_fg(color);
                        }
                    }
                }
            }
        }
    }
}

/// Generates a random character for the matrix rain drop.
///
/// ⚡ Bolt Optimization: Uses a byte string literal (`b"..."`) instead of a regular
/// string to allow O(1) zero-cost indexing via `chars[idx]`. This replaces the previous
/// O(N) `chars().nth(idx)` traversal, eliminating iterator overhead and speeding up
/// the tight rendering loop without unsafe code, since all candidates are ASCII.
fn random_char() -> char {
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$+-*/=<>^%&?@#";
    let idx = rand::thread_rng().gen_range(0..chars.len());
    chars[idx] as char
}
