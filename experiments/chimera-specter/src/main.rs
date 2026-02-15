mod audio;
mod fluid;

use audio::AudioSystem;
use chimera_lang::prelude::*;
use fluid::FluidSolver;
use macroquad::prelude::*;

const FLUID_SIZE: usize = 128;
const AGENT_COUNT: usize = 100;

struct SpectralAgent {
    vm: ChimeraVM,
    x: f32,
    y: f32,
    target_freq_bin: usize,
    energy: f32,
    color: Color,
}

impl SpectralAgent {
    fn new(x: f32, y: f32, freq_bin: usize) -> Self {
        // Minimal DNA: Just stay alive
        let genes = vec![
            Gene {
                op: OpCode::Nop,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let vm = ChimeraVM::new(dna);

        // Color based on frequency
        // Low freq = Red, High freq = Blue
        let hue = freq_bin as f32 / 256.0; // spectrum usually 512, but useful range is lower
        let color = hsl_to_rgb(hue.clamp(0.0, 1.0), 1.0, 0.5);

        Self {
            vm,
            x,
            y,
            target_freq_bin: freq_bin,
            energy: 50.0,
            color,
        }
    }

    fn update(&mut self, fluid: &FluidSolver, spectrum: &[f32], dt: f32) {
        // 1. Move with fluid
        let ix = (self.x as usize).clamp(0, FLUID_SIZE - 1);
        let iy = (self.y as usize).clamp(0, FLUID_SIZE - 1);
        let idx = ix + iy * FLUID_SIZE;

        let vx = fluid.vx[idx];
        let vy = fluid.vy[idx];

        // Advection
        self.x += vx * dt * 50.0; // Scale velocity
        self.y += vy * dt * 50.0;

        // Swim a bit (Brownian motion)
        self.x += macroquad::rand::gen_range(-0.1, 0.1);
        self.y += macroquad::rand::gen_range(-0.1, 0.1);

        // Boundary wrap
        if self.x < 0.0 {
            self.x = FLUID_SIZE as f32 - 0.1;
        }
        if self.x >= FLUID_SIZE as f32 {
            self.x = 0.1;
        }
        if self.y < 0.0 {
            self.y = FLUID_SIZE as f32 - 0.1;
        }
        if self.y >= FLUID_SIZE as f32 {
            self.y = 0.1;
        }

        // 2. Eat Spectral Energy
        if self.target_freq_bin < spectrum.len() {
            let food = spectrum[self.target_freq_bin];
            if food > 0.1 {
                self.energy += food * 5.0; // Gain energy
            }
        }

        // 3. Burn Energy
        self.energy -= 0.1;

        // 4. Run VM (Metabolic cost)
        // In a full simulation, VM output would drive movement.
        // For now, it just cycles.
        self.vm.step();
    }
}

// Helper for HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}

#[macroquad::main("Chimera Specter")]
async fn main() -> anyhow::Result<()> {
    // Init Audio
    let mut audio = AudioSystem::new()?;

    // Init Fluid
    let mut fluid = FluidSolver::new(FLUID_SIZE, 0.1, 0.00001, 0.00001);

    // Init Agents
    let mut agents: Vec<SpectralAgent> = (0..AGENT_COUNT)
        .map(|_| {
            SpectralAgent::new(
                macroquad::rand::gen_range(0.0, FLUID_SIZE as f32),
                macroquad::rand::gen_range(0.0, FLUID_SIZE as f32),
                macroquad::rand::gen_range(0, 100), // Start preferring low freqs
            )
        })
        .collect();

    // Init Texture
    let mut image = Image::gen_image_color(FLUID_SIZE as u16, FLUID_SIZE as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Linear);

    loop {
        clear_background(BLACK);

        // --- Input ---
        let spectrum = audio.get_spectrum();

        // Feed Audio to Fluid
        for i in 0..FLUID_SIZE {
            let bin_idx = (i * spectrum.len()) / FLUID_SIZE;
            if bin_idx < spectrum.len() {
                let val = spectrum[bin_idx];
                if val > 0.1 {
                    fluid.add_density(i, FLUID_SIZE - 2, val * 5.0);
                    fluid.add_velocity(i, FLUID_SIZE - 2, 0.0, -val * 2.0);
                }
            }
        }

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();
            let fx = (mx / sw * FLUID_SIZE as f32) as usize;
            let fy = (my / sh * FLUID_SIZE as f32) as usize;

            if fx < FLUID_SIZE && fy < FLUID_SIZE {
                fluid.add_density(fx, fy, 50.0);
                fluid.add_velocity(fx, fy, macroquad::rand::gen_range(-1.0, 1.0), macroquad::rand::gen_range(-1.0, 1.0));
            }
        }

        // --- Physics ---
        fluid.step();

        // Update Agents
        let mut new_agents = Vec::new();
        agents.retain_mut(|agent| {
            agent.update(&fluid, &spectrum, 0.1);

            // Reproduction
            if agent.energy > 100.0 {
                agent.energy /= 2.0;
                let mut child = SpectralAgent::new(agent.x, agent.y, agent.target_freq_bin);
                child.energy = agent.energy;

                // Mutation: Shift frequency preference
                if macroquad::rand::gen_range(0.0, 1.0) < 0.2 {
                    let shift = macroquad::rand::gen_range(-5, 6);
                    child.target_freq_bin = (child.target_freq_bin as i32 + shift).clamp(0, 511) as usize;
                    // Recalculate color
                    let hue = child.target_freq_bin as f32 / 256.0;
                    child.color = hsl_to_rgb(hue.clamp(0.0, 1.0), 1.0, 0.5);
                }
                new_agents.push(child);
            }

            agent.energy > 0.0
        });
        agents.append(&mut new_agents);

        // Cap population
        if agents.len() > 1000 {
            agents.truncate(1000);
        }

        // --- Render ---
        // Fluid Background
        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                let idx = x + y * FLUID_SIZE;
                let d = fluid.density[idx];
                let r = (d * 2.0).min(0.5); // Dimmer fluid
                let g = (d * 1.0).min(0.5);
                let b = (d * 3.0).min(0.8);
                image.set_pixel(x as u32, y as u32, Color::new(r, g, b, 1.0));
            }
        }
        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Agents
        let sw = screen_width();
        let sh = screen_height();
        for agent in &agents {
            let sx = (agent.x / FLUID_SIZE as f32) * sw;
            let sy = (agent.y / FLUID_SIZE as f32) * sh;
            let size = (agent.energy / 50.0).clamp(2.0, 10.0);
            draw_circle(sx, sy, size, agent.color);
        }

        // HUD
        draw_text("Chimera Specter", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", agents.len()), 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 80.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
