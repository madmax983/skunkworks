use rand::Rng;
use ratatui::{buffer::Buffer, layout::Rect, style::Color};

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
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            width: 0,
            height: 0,
        }
    }

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

fn random_char() -> char {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$+-*/=<>^%&?@#";
    let idx = rand::thread_rng().gen_range(0..chars.len());
    chars.chars().nth(idx).unwrap_or('?')
}
