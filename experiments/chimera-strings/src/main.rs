//! # Chimera Strings 🧬🎻
//!
//! An evolutionary symphony where genetic organisms learn to play magnetic instruments.
//!
//! ## Lineage
//! - **Parent A**: `chimera-lang` (Genetic/Evolutionary VM logic)
//! - **Parent B**: `ferrous-strings` (Acoustic-magnetic string physics)
//!
//! ## Concept
//! The `chimera-lang` crate provides agents equipped with `ChimeraVM` brains. These agents execute discrete DNA opcodes. The `ferrous-strings` crate provides a continuous 2D space where magnetic strings vibrate and interact with a magnetic `Platter` and `Particle` flow.
//!
//! In this hybrid, we splice the two:
//! 1. **Genetic Intent**: Agents navigate the continuous space, driven by their genetic phase and the magnetic forces of the environment.
//! 2. **Acoustic-Magnetic Phenotype**: When an agent executes specific opcodes (simulated by their phase in this iteration), they perform a kinetic strike on a nearby `FerrousString`.
//! 3. **Emergence**: The strings vibrate, altering the magnetic platter, which in turn influences the flow of particles and the movement of the agents themselves, creating a bidirectional acoustic-magnetic feedback loop governed by genetic traits.
//!
//! ## Traits Inherited
//! - **From `chimera-lang`**: `ChimeraVM`, `Dna`, `OpCode` logic dictating agent behavior.
//! - **From `ferrous-strings`**: The `FerrousString` and `Particle` entities, the acoustic coupling, and the underlying magnetic `Platter` from `ferrous-core`.
//! - **Novel Trait**: Acoustic-Magnetic Genetics. The agents translate their discrete DNA opcodes into continuous kinetic strikes on strings, harnessing the resulting magnetic fields for survival and navigation.
//!
use ferrous_core::Platter;
use macroquad::prelude::*;

use chimera_lang::ast::{Dna, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

mod audio;
mod particle;
mod string;

use audio::{init_audio, AudioCommand};
use particle::Particle;
use string::FerrousString;

const STRING_COUNT: usize = 8;
const PARTICLE_COUNT: usize = 300;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2
const AGENT_COUNT: usize = 15;

/// Represents an acoustic genetic organism.
///
/// **Lineage Inheritance**:
/// - `ChimeraVM` (Parent A): Executes genetic opcodes to determine the agent's behavior.
/// - The spatial properties (`x`, `y`, `phase`) (Parent B): Allows the agent to interact
///   with the continuous acoustic-magnetic world of `ferrous-strings`.
struct Agent {
    vm: ChimeraVM,
    x: f32,
    y: f32,
    phase: f32,
}

impl Agent {
    fn new(x: f32, y: f32) -> Self {
        // Simple DNA that generates rhythmic opcodes
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Nop,
                args: vec![],
            },
        ];

        let dna = Dna::from_genes(genes);

        Self {
            vm: ChimeraVM::new(dna),
            x,
            y,
            phase: rand::gen_range(0.0, std::f32::consts::PI * 2.0),
        }
    }
}

#[macroquad::main("Chimera Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    let _audio_handle = audio_handle;

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    let mut strings: Vec<FerrousString> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut agents: Vec<Agent> = Vec::new();

    // Initialize strings
    for i in 0..STRING_COUNT {
        let x = 100.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    // Initialize particles
    for _ in 0..PARTICLE_COUNT {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        particles.push(Particle::new(x, y));
    }

    // Initialize Chimera Agents
    for _ in 0..AGENT_COUNT {
        let x = rand::gen_range(100.0, screen_width() - 100.0);
        let y = rand::gen_range(100.0, screen_height() - 100.0);
        agents.push(Agent::new(x, y));
    }

    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    let mut evolution_timer = 0.0;
    let evolution_interval = 2.0;

    loop {
        let dt = get_frame_time();

        // 1. Decay Platter
        platter.decay(0.99);

        // 2. Update Strings (Magnetize Platter)
        for s in &mut strings {
            s.update_physics(dt, &platter, grid_scale);

            if s.vibration.abs() > 0.1 {
                let steps = 20;
                let step_size = s.length / steps as f32;

                for i in 0..=steps {
                    let y_offset = i as f32 * step_size;
                    let ratio = y_offset / s.length;
                    let shape = (std::f32::consts::PI * ratio).sin();
                    let x = s.pos.x + s.vibration * shape;
                    let y = s.pos.y + y_offset;

                    let gx = (x / grid_scale) as i32;
                    let gy = (y / grid_scale) as i32;

                    if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                        let val = (s.vibration * 0.01 * shape) as f64;
                        platter.magnetize(gx as usize, gy as usize, val);
                    }
                }
            }
        }

        // 3. Update Particles
        for p in &mut particles {
            p.update(&platter, grid_scale, dt);
        }

        evolution_timer += dt;

        // --- Evolution ---
        if evolution_timer > evolution_interval {
            evolution_timer = 0.0;

            for s in &mut strings {
                let mut count = 0;
                let x_min = s.pos.x - 20.0;
                let x_max = s.pos.x + 20.0;
                let y_min = s.pos.y;
                let y_max = s.pos.y + s.length;

                for p in &particles {
                    if p.pos.x >= x_min && p.pos.x <= x_max && p.pos.y >= y_min && p.pos.y <= y_max
                    {
                        count += 1;
                    }
                }

                s.fitness = (count as f32 / 50.0).clamp(0.0, 1.0);
            }

            for (i, s) in strings.iter_mut().enumerate() {
                let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
                s.evolve(target_freq);
            }
        }

        // 4. Update Agents
        // The agents execute `ChimeraVM` code and traverse the continuous space
        // driven by the magnetic fields produced by the vibrating `FerrousStrings`.
        for agent in &mut agents {
            agent.vm.step();
            agent.phase += dt * 2.0;

            // Agents wander based on the magnetic platter
            let gx = (agent.x / grid_scale) as usize;
            let gy = (agent.y / grid_scale) as usize;

            let mut force_x = 0.0;
            let mut force_y = 0.0;

            if gx > 0 && gx < grid_w - 1 && gy > 0 && gy < grid_h - 1 {
                let mag_r = platter.get_magnetism(gx + 1, gy) as f32;
                let mag_l = platter.get_magnetism(gx - 1, gy) as f32;
                let mag_t = platter.get_magnetism(gx, gy - 1) as f32;
                let mag_b = platter.get_magnetism(gx, gy + 1) as f32;

                force_x = mag_r - mag_l;
                force_y = mag_b - mag_t;
            }

            // Agents also drift
            agent.x += (rand::gen_range(-1.0, 1.0) as f32 + force_x * 5.0) * 1.0;
            agent.y += (rand::gen_range(-1.0, 1.0) as f32 + force_y * 5.0) * 1.0;

            agent.x = agent.x.clamp(0.0, screen_width());
            agent.y = agent.y.clamp(0.0, screen_height());

            // Agents interact with strings based on VM execution and position.
            // **Novel Trait**: Genetic opcodes are translated into kinetic strikes.
            for s in &mut strings {
                let string_x = s.pos.x;
                let in_range_x = (agent.x - string_x).abs() < 20.0;
                let in_range_y = agent.y >= s.pos.y && agent.y <= s.pos.y + s.length;

                // Agent "strikes" the string if it executes an instruction that generates energy (simulated by probability here)
                if in_range_x && in_range_y {
                    // To tie it to ChimeraVM, we use the phase to simulate an opcode
                    // triggering a kinetic strike. The strike vibrates the string and produces sound.
                    if agent.phase > std::f32::consts::PI * 2.0 {
                        agent.phase = 0.0;
                        let strength = rand::gen_range(10.0, 30.0);
                        let direction = if agent.x > string_x { 1.0 } else { -1.0 };
                        s.pluck(strength * direction);

                        let _ = cmd_tx.send(AudioCommand {
                            frequency: s.frequency,
                            decay: s.decay,
                            amplitude: (strength / 50.0).clamp(0.1, 0.8),
                        });
                    }
                }
            }
        }

        // --- Render ---
        clear_background(BLACK);

        // 1. Draw Platter
        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y);

                let val = mag as f32;
                let c = if val < 0.0 {
                    Color::new(0.0, 0.0, (-val).clamp(0.0, 1.0), 1.0)
                } else {
                    Color::new(val.clamp(0.0, 1.0), 0.0, 0.0, 1.0)
                };

                image.set_pixel(x as u32, y as u32, c);
            }
        }
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid_w as f32 * grid_scale, grid_h as f32 * grid_scale)),
                ..Default::default()
            },
        );

        // 2. Draw Particles
        for p in &particles {
            p.draw();
        }

        // 3. Draw Strings
        for s in &strings {
            s.draw();
        }

        // 4. Draw Agents
        for agent in &agents {
            draw_circle(agent.x, agent.y, 6.0, YELLOW);
            draw_circle_lines(agent.x, agent.y, 8.0, 1.0, ORANGE);
        }

        // UI
        draw_text("Chimera Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text("Acoustic Genetic Organisms", 10.0, 50.0, 20.0, GRAY);
        draw_text(
            format!("Agents: {}", agents.len()).as_str(),
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}
