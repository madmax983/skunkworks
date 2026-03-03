mod simulation;

use macroquad::prelude::*;
use simulation::{Platter, Rgba};

enum HeadMode {
    Passive,     // Just looks, doesn't write back
    Refresh,     // Reads and writes back (locking in decay)
    Recover,     // Reads, smooths/guesses, and writes back (locking in hallucination)
    Destructive, // Writes random noise
}

impl HeadMode {
    fn next(&self) -> Self {
        match self {
            HeadMode::Passive => HeadMode::Refresh,
            HeadMode::Refresh => HeadMode::Recover,
            HeadMode::Recover => HeadMode::Destructive,
            HeadMode::Destructive => HeadMode::Passive,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            HeadMode::Passive => "PASSIVE (Observer)",
            HeadMode::Refresh => "REFRESH (The Echo)",
            HeadMode::Recover => "RECOVER (The Hallucination)",
            HeadMode::Destructive => "DESTRUCTIVE (The Eraser)",
        }
    }
}

struct Head {
    x: usize,
    y: usize,
    mode: HeadMode,
    speed: usize,
}

impl Head {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            mode: HeadMode::Refresh,
            speed: 1,
        }
    }

    fn update(&mut self, platter: &mut Platter) {
        for _ in 0..self.speed {
            // Process current sector
            match self.mode {
                HeadMode::Passive => {
                    // Do nothing
                }
                HeadMode::Refresh => {
                    if let Some(color) = platter.read(self.x, self.y) {
                        platter.write(self.x, self.y, color);
                    }
                }
                HeadMode::Recover => {
                    // Simple recovery: Average of neighbors
                    // We need to read neighbors *before* writing, but Platter doesn't allow simultaneous borrow easily if we adhere strictly to safety?
                    // Actually, Platter::read is immutable, write is mutable. We can read then write.

                    let mut sum_r = 0.0;
                    let mut sum_g = 0.0;
                    let mut sum_b = 0.0;
                    let mut count = 0.0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = self.x as isize + dx;
                            let ny = self.y as isize + dy;
                            if nx >= 0 && ny >= 0 {
                                if let Some(c) = platter.read(nx as usize, ny as usize) {
                                    sum_r += c.r;
                                    sum_g += c.g;
                                    sum_b += c.b;
                                    count += 1.0;
                                }
                            }
                        }
                    }

                    if count > 0.0 {
                        let avg_color = Rgba::new(sum_r / count, sum_g / count, sum_b / count, 1.0);
                        platter.write(self.x, self.y, avg_color);
                    }
                }
                HeadMode::Destructive => {
                    platter.write(
                        self.x,
                        self.y,
                        Rgba::new(
                            macroquad::rand::gen_range(0.0, 1.0),
                            macroquad::rand::gen_range(0.0, 1.0),
                            macroquad::rand::gen_range(0.0, 1.0),
                            1.0,
                        ),
                    );
                }
            }

            // Move head
            self.x += 1;
            if self.x >= platter.width() {
                self.x = 0;
                self.y += 1;
            }
            if self.y >= platter.height() {
                self.y = 0;
            }
        }
    }
}

#[macroquad::main("Magnetic Echo")]
async fn main() {
    let mut platter = Platter::new(64, 64);
    let mut head = Head::new();
    let mut paused = false;

    // Seed some data
    for y in 20..44 {
        for x in 20..44 {
            platter.write(x, y, Rgba::new(1.0, 0.0, 0.0, 1.0)); // Red square
        }
    }
    // Blue circleish
    for y in 0..64 {
        for x in 0..64 {
            let dx = x as f32 - 32.0;
            let dy = y as f32 - 32.0;
            if dx * dx + dy * dy < 100.0 {
                platter.write(x, y, Rgba::new(0.0, 0.0, 1.0, 1.0));
            }
        }
    }

    loop {
        // Handle Input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            platter = Platter::new(64, 64);
            // Re-Seed
            for y in 0..64 {
                for x in 0..64 {
                    let dx = x as f32 - 32.0;
                    let dy = y as f32 - 32.0;
                    if dx * dx + dy * dy < 100.0 {
                        platter.write(x, y, Rgba::new(0.0, 0.0, 1.0, 1.0));
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::M) {
            head.mode = head.mode.next();
        }

        // Mouse interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let w = screen_width();
            let h = screen_height();
            let cell_w = w / platter.width() as f32;
            let cell_h = h / platter.height() as f32;

            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;

            if gx < platter.width() && gy < platter.height() {
                // Paint rainbow
                let t = get_time() as f32;
                let r = (t.sin() + 1.0) / 2.0;
                let g = ((t + 2.0).sin() + 1.0) / 2.0;
                let b = ((t + 4.0).sin() + 1.0) / 2.0;
                platter.write(gx, gy, Rgba::new(r, g, b, 1.0));
            }
        }

        // Update
        if !paused {
            platter.decay(0.005); // Decay rate
            platter.drift(0.1); // Drift/Noise amount
            head.update(&mut platter);
            head.update(&mut platter); // Speed up head x2
            head.update(&mut platter); // Speed up head x3
        }

        // Draw
        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();
        let cell_w = w / platter.width() as f32;
        let cell_h = h / platter.height() as f32;

        for y in 0..platter.height()() {
            for x in 0..platter.width()() {
                if let Some(color) = platter.read(x, y) {
                    let c = color;
                    // Render using macroquad color
                    // We can visualize magnetism as alpha or size?
                    // Let's use color directly, but maybe darken if magnetism is low?
                    // Actually, drift changes the color itself.

                    // Let's modify visualization based on magnetism too, so we see "weak" bits
                    // Or keep it pure to see what the "data" looks like.
                    // Let's make it pure.

                    draw_rectangle(
                        x as f32 * cell_w,
                        y as f32 * cell_h,
                        cell_w,
                        cell_h,
                        Color::new(c.r, c.g, c.b, 1.0),
                    );
                }
            }
        }

        // Draw Head
        draw_rectangle(
            head.x as f32 * cell_w,
            head.y as f32 * cell_h,
            cell_w,
            cell_h,
            RED, // Head is always bright red
        );
        draw_rectangle_lines(
            head.x as f32 * cell_w,
            head.y as f32 * cell_h,
            cell_w,
            cell_h,
            2.0,
            WHITE,
        );

        // Draw UI
        draw_text("Magnetic Echo", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Mode: {}", head.mode.as_str()),
            10.0,
            60.0,
            20.0,
            GOLD,
        );
        draw_text(
            "[Space] Pause | [R] Reset | [M] Mode | [Click] Inject",
            10.0,
            80.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
