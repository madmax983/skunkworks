use crate::lbm::Fluid;
use font8x8::legacy::BASIC_LEGACY;

pub struct Scroller {
    text: String,
    scroll_offset: f32, // float for smooth speed control
    scroll_speed: f32,
    y_pos: usize, // Vertical position of the text baseline
}

impl Scroller {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            text: String::from("  TYPE TO ADD OBSTACLES  "),
            scroll_offset: -(width as f32), // Start off-screen or partial? Start at right.
            scroll_speed: 0.5,              // 0.5 cells per frame
            y_pos: height / 2 - 4,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn pop_char(&mut self) {
        self.text.pop();
    }

    pub fn tick(&mut self, fluid: &mut Fluid) {
        // Move text
        self.scroll_offset -= self.scroll_speed;

        // Loop text for infinite scroll effect if desired, or just let it pass
        // Let's loop it if it goes too far left.
        // Approx width of text = len * 8.
        let text_width = self.text.len() * 8;
        if self.scroll_offset < -(text_width as f32) {
            self.scroll_offset = fluid.width as f32;
        }

        // Render to fluid
        fluid.clear_obstacles();

        let start_x = self.scroll_offset as i32;
        let y_base = self.y_pos;

        for (i, c) in self.text.chars().enumerate() {
            let char_x = start_x + (i * 8) as i32;

            // Optimization: Skip if completely off-screen
            if char_x > fluid.width as i32 || char_x + 8 < 0 {
                continue;
            }

            if let Some(glyph) = BASIC_LEGACY.get(c as usize) {
                // glyph is [u8; 8]
                // Each u8 is a row. Top to bottom?
                // Usually font8x8 is: byte 0 is top row.
                // LSB is left? or MSB is left?
                // font8x8: 0x80 is left-most pixel.

                for (row, &byte) in glyph.iter().enumerate() {
                    for col in 0..8 {
                        // bit 0 is right-most (x+7). bit 7 is left-most (x+0).
                        if (byte & (1 << col)) != 0 {
                            let px = char_x + (7 - col);
                            let py = y_base + row;

                            if px >= 0 && px < fluid.width as i32 && py < fluid.height {
                                fluid.add_obstacle(px as usize, py);
                            }
                        }
                    }
                }
            }
        }
    }
}
