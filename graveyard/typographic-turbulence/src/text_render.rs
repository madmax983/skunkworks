use crate::lbm::{FluidSim, HEIGHT, WIDTH};
use rusttype::{Font, Point, Scale};

pub struct FloatingText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub scale: f32,
}

pub struct TextManager {
    font: Font<'static>,
    pub texts: Vec<FloatingText>,
}

impl TextManager {
    pub fn new() -> Self {
        let font_data = include_bytes!("../assets/font.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
        Self {
            font,
            texts: Vec::new(),
        }
    }

    pub fn spawn(&mut self, text: &str, x: f32, y: f32, vx: f32, vy: f32) {
        self.texts.push(FloatingText {
            text: text.to_string(),
            x,
            y,
            vx,
            vy,
            scale: 24.0,
        });
    }

    pub fn update(&mut self, sim: &mut FluidSim, dt: f32) {
        // 1. Move texts
        for text in &mut self.texts {
            text.x += text.vx * dt;
            text.y += text.vy * dt;

            // Wrap around
            // Heuristic for length:
            let approx_width = text.text.len() as f32 * text.scale * 0.5;
            if text.x > WIDTH as f32 {
                text.x = -approx_width;
            } else if text.x < -approx_width {
                text.x = WIDTH as f32;
            }
        }

        // 2. Clear obstacles
        sim.clear_obstacles();

        // 3. Rasterize to obstacles
        for text in &self.texts {
            let scale = Scale::uniform(text.scale);
            let v_metrics = self.font.v_metrics(scale);

            let start = Point {
                x: text.x,
                y: text.y + v_metrics.ascent,
            };

            for glyph in self.font.layout(&text.text, scale, start) {
                if let Some(bb) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, v| {
                        if v > 0.5 {
                            let px = bb.min.x + gx as i32;
                            let py = bb.min.y + gy as i32;

                            if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                                let ux = px as usize;
                                let uy = py as usize;

                                sim.set_obstacle(ux, uy, true);
                                // Inject momentum (moving boundary effect)
                                // We add velocity to the fluid at this point.
                                // Since add_velocity adds to a 3x3 kernel, calling it per pixel is a bit expensive but robust.
                                // Let's simplify: just add to the specific cell?
                                // FluidSim::add_velocity adds to 3x3.
                                // Let's add less amount since we do it for every pixel.
                                // Or we can create a new method `set_velocity_at(x, y, u, v)` in FluidSim.
                                // But `add_velocity` is fine.
                                sim.add_velocity(ux, uy, text.vx * 0.1, text.vy * 0.1);
                            }
                        }
                    });
                }
            }
        }
    }
}
