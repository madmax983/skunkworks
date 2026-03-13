//! # Digital Rain 🌧️
//!
//! Provides the iconic cascading green code effect ("Matrix Rain") for the Chimera TUI.
//!
//! This module renders streams of characters that fall vertically down the screen,
//! creating an atmospheric visual effect. It is primarily used during transitions,
//! loading screens, or special execution states within the VM.
//!
//! ## Visual Mechanics
//!
//! The rain is composed of independent `Column`s. Each column:
//!
//! *   Has a randomized `speed` (how fast it falls).
//! *   Has a randomized `len` (how many characters make up the tail).
//! *   Periodically mutates its characters to simulate changing data.
//! *   Features a bright white "head" character, followed by a light green body,
//!     and a dark gray trailing tail.
//!
//! ## Example
//!
//! ```
//! use chimera_lang::matrix_rain::MatrixRain;
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! // 1. Initialize the effect state
//! let mut rain = MatrixRain::new();
//!
//! // 2. Update the physics simulation (call this once per frame)
//! //    Provide the current screen dimensions to spawn columns correctly.
//! rain.update(80, 24);
//!
//! // 3. Render the effect onto a ratatui Buffer
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! rain.render(&mut buf, area);
//! ```

use rand::Rng;
use ratatui::{buffer::Buffer, layout::Rect, style::Color};

/// The state engine for the Matrix Rain visual effect.
///
/// Manages a collection of falling character `Column`s and handles their
/// physics (falling speed), mutation (changing characters), and rendering
/// onto a `ratatui` terminal buffer.
pub struct MatrixRain {
    /// The active, falling columns of characters on the screen.
    columns: Vec<Column>,
    /// The cached width of the terminal (used to bound spawning).
    width: u16,
    /// The cached height of the terminal (used for culling off-screen columns).
    height: u16,
}

/// A single vertical stream of characters falling down the screen.
struct Column {
    /// The horizontal X-coordinate where this column falls.
    x: u16,
    /// The vertical Y-coordinate (floating point for smooth speed scaling).
    y: f32,
    /// How many units the column moves downward per frame.
    speed: f32,
    /// The actual sequence of characters making up the column.
    chars: Vec<char>,
    /// The total length of the column's tail.
    len: usize,
}

impl Default for MatrixRain {
    fn default() -> Self {
        Self::new()
    }
}

impl MatrixRain {
    /// Constructs a new, empty `MatrixRain` engine.
    ///
    /// The engine begins with no active columns. Columns will begin spawning
    /// automatically once [`MatrixRain::update`] is called with non-zero dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use chimera_lang::matrix_rain::MatrixRain;
    /// let rain = MatrixRain::new();
    /// ```
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    /// Advances the rain simulation by one frame.
    ///
    /// This method performs several actions:
    /// 1.  **Spawning:** Randomly generates new columns at the top of the screen if density allows.
    /// 2.  **Physics:** Moves existing columns downward based on their individual speeds.
    /// 3.  **Mutation:** Randomly scrambles characters within existing columns to simulate changing code.
    /// 4.  **Culling:** Removes columns that have completely fallen off the bottom of the screen.
    ///
    /// # Parameters
    ///
    /// *   `width`: The current width of the terminal viewport (used for spawn limits).
    /// *   `height`: The current height of the terminal viewport (used for culling).
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

    /// Spawns a new column of characters at the specified horizontal position.
    ///
    /// The new column will start at `y = 0.0` with a random length, speed, and
    /// initial set of characters.
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

    /// Draws the current state of the falling rain onto a `ratatui` [`Buffer`].
    ///
    /// This method paints the characters into the terminal buffer, applying
    /// the signature color styling:
    /// -   **Head:** Bright White
    /// -   **Body:** Light Green
    /// -   **Tail:** Dark Gray
    ///
    /// # Panics
    ///
    /// This function is safe and performs bounds-checking to ensure it never
    /// writes outside the provided `area` or the underlying `buf`.
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

/// Generates a random alphanumeric or symbol character suitable for the digital rain.
///
/// To prevent unnecessary heap allocations from splitting strings into `Vec<char>`,
/// we iterate over `chars()` directly.
fn random_char() -> char {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$+-*/=<>^%&?@#";
    let idx = rand::thread_rng().gen_range(0..chars.len());
    chars.chars().nth(idx).unwrap_or('?')
}
