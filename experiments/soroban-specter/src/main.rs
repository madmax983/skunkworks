mod fluid;

use fluid::FluidSolver;
use macroquad::prelude::*;
use soroban::Soroban;

const FLUID_SIZE: usize = 128;
const NUM_COLS: usize = 13;

struct SorobanSpecter {
    soroban: Soroban,
    fluid: FluidSolver,
    image: Image,
    texture: Texture2D,
    last_update: f64,
    auto_mode: bool,
}

impl SorobanSpecter {
    fn new() -> Self {
        let fluid = FluidSolver::new(FLUID_SIZE, 0.1, 0.00001, 0.00001);
        let image = Image::gen_image_color(FLUID_SIZE as u16, FLUID_SIZE as u16, BLACK);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);

        Self {
            soroban: Soroban::new(),
            fluid,
            image,
            texture,
            last_update: get_time(),
            auto_mode: true,
        }
    }

    fn update(&mut self) {
        // Auto-add random numbers
        if self.auto_mode && get_time() - self.last_update > 0.5 {
            let val = macroquad::rand::gen_range(1u64, 1000u64);
            // occasional subtraction
            if macroquad::rand::gen_range(0, 5) == 0 {
                 // Simple subtraction check to avoid underflow visualization mess for now
                 // (Soroban handles borrow, but let's keep it simple)
                 let current = self.soroban.value();
                 if current > val {
                     self.soroban.sub(val);
                 } else {
                     self.soroban.add(val);
                 }
            } else {
                self.soroban.add(val);
            }
            self.last_update = get_time();

            // "Splash" effect on change
            // We could track changed columns, but let's just splash globally for now or based on active beads
        }

        // Map Soroban State to Fluid
        // Columns map to X
        let col_width = FLUID_SIZE as f32 / NUM_COLS as f32;
        let beam_y = FLUID_SIZE as f32 / 3.0;

        for (i, col) in self.soroban.columns.iter().enumerate() {
            let cx = (i as f32 * col_width + col_width / 2.0) as usize;

            // Heaven Bead (Value 5)
            // Active = Down (near beam). Inactive = Up.
            let heaven_y = if col.upper_active {
                beam_y - 5.0
            } else {
                beam_y - 15.0
            };

            // Inject density for Heaven Bead
            if col.upper_active {
                // High density for active
                self.fluid.add_density(cx, heaven_y as usize, 5.0);
                // Downward velocity (gravity/weight)
                self.fluid.add_velocity(cx, heaven_y as usize, 0.0, 0.5);
            } else {
                 // Faint density for inactive
                self.fluid.add_density(cx, heaven_y as usize, 0.5);
            }

            // Earth Beads (Value 1)
            // Active = Up (near beam). Inactive = Down.
            // We have 4 beads.
            // If lower_active is N, then N beads are up.

            for b in 0..4 {
                let is_active = b < col.lower_active;
                let bead_y = if is_active {
                    // Active beads stack up against the beam
                    beam_y + 5.0 + (b as f32 * 3.0)
                } else {
                    // Inactive beads stack down at the bottom
                    (FLUID_SIZE as f32) - 10.0 - ((3 - b) as f32 * 3.0)
                };

                if is_active {
                    self.fluid.add_density(cx, bead_y as usize, 2.0);
                    // Upward velocity (buoyancy/magnetic pull to beam)
                    self.fluid.add_velocity(cx, bead_y as usize, 0.0, -0.5);
                } else {
                     self.fluid.add_density(cx, bead_y as usize, 0.2);
                }
            }
        }

        // Step Fluid
        self.fluid.step();

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            let fx = (mx / sw * FLUID_SIZE as f32) as usize;
            let fy = (my / sh * FLUID_SIZE as f32) as usize;

            if fx < FLUID_SIZE && fy < FLUID_SIZE {
                self.fluid.add_density(fx, fy, 50.0);
                self.fluid.add_velocity(fx, fy, macroquad::rand::gen_range(-2.0, 2.0), macroquad::rand::gen_range(-2.0, 2.0));
            }
        }

        if is_key_pressed(KeyCode::Space) {
            self.auto_mode = !self.auto_mode;
        }

        if is_key_pressed(KeyCode::C) {
             self.soroban = Soroban::new(); // Clear
        }
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        // Update Texture
        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                let idx = x + y * FLUID_SIZE;
                let d = self.fluid.density[idx];

                // Color mapping
                // Blue-ish fluid, getting white with high density
                let r = (d * 0.5).min(1.0);
                let g = (d * 0.8).min(1.0);
                let b = (d * 1.5).min(1.0);
                let a = 1.0;

                self.image.set_pixel(x as u32, y as u32, Color::new(r, g, b, a));
            }
        }
        self.texture.update(&self.image);

        // Draw Fluid
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Soroban Overlay
        let sw = screen_width();
        let sh = screen_height();
        let col_width_px = sw / NUM_COLS as f32;
        let beam_y_px = sh / 3.0;

        // Draw Beam
        draw_line(0.0, beam_y_px, sw, beam_y_px, 2.0, GOLD);

        // Draw Rods
        for i in 0..NUM_COLS {
            let x = i as f32 * col_width_px + col_width_px / 2.0;
            draw_line(x, 0.0, x, sh, 1.0, Color::new(1.0, 0.84, 0.0, 0.3)); // Faint gold
        }

        // Draw Value
        let val_str = format!("{}", self.soroban.value());
        let font_size = 40.0;
        let text_dims = measure_text(&val_str, None, font_size as u16, 1.0);
        draw_text(&val_str, sw - text_dims.width - 20.0, sh - 20.0, font_size, WHITE);

        // Instructions
        draw_text("Soroban Specter", 20.0, 30.0, 30.0, WHITE);
        draw_text("Space: Toggle Auto | Click: Splash | C: Clear", 20.0, 60.0, 20.0, LIGHTGRAY);
    }
}

#[macroquad::main("Soroban Specter")]
async fn main() -> anyhow::Result<()> {
    let mut specter = SorobanSpecter::new();

    loop {
        specter.update();
        specter.draw();
        next_frame().await;
    }
}
