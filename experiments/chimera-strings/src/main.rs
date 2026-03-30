use macroquad::prelude::*;
use std::f32::consts::PI;

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

use ferrous_core::Platter;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const NUM_STRINGS: usize = 12;
const GRID_SCALE: f32 = 4.0;

struct StringData {
    x: f32,
    y1: f32,
    y2: f32,
    amplitude: f32,
    frequency: f32,
    phase: f32,
    decay: f32,
}

impl StringData {
    fn new(x: f32) -> Self {
        Self {
            x,
            y1: 100.0,
            y2: HEIGHT - 100.0,
            amplitude: 0.0,
            frequency: rand::gen_range(2.0, 10.0),
            phase: rand::gen_range(0.0, PI * 2.0),
            decay: rand::gen_range(0.95, 0.99),
        }
    }

    fn update(&mut self, dt: f32) {
        self.phase += self.frequency * dt * 10.0;
        self.amplitude *= self.decay;
    }

    fn pluck(&mut self, amount: f32) {
        self.amplitude += amount;
        self.amplitude = self.amplitude.clamp(-50.0, 50.0);
    }
}

struct Agent {
    vm: ChimeraVM,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    _dna: Dna,
    energy: f32,
    age: usize,
}

impl Agent {
    fn new(x: f32, y: f32) -> Self {
        // Random simple genetics
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(rand::gen_range(1, 10))],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![chimera_lang::ast::Strand { genes }],
            },
            evolution_config: None,
        };

        Self {
            vm: ChimeraVM::new(dna.clone()),
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            _dna: dna,
            energy: 100.0,
            age: 0,
        }
    }
}

#[macroquad::main("Chimera Strings")]
async fn main() {
    let mut strings = Vec::new();
    let spacing = WIDTH / (NUM_STRINGS as f32 + 1.0);
    for i in 1..=NUM_STRINGS {
        strings.push(StringData::new(i as f32 * spacing));
    }

    let mut agents = Vec::new();
    for _ in 0..50 {
        agents.push(Agent::new(
            rand::gen_range(50.0, WIDTH - 50.0),
            rand::gen_range(50.0, HEIGHT - 50.0),
        ));
    }

    let mut platter = Platter::new(
        (WIDTH / GRID_SCALE) as usize,
        (HEIGHT / GRID_SCALE) as usize,
    );

    loop {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Space) {
            agents.clear();
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mpos = mouse_position();
            for s in &mut strings {
                if (mpos.0 - s.x).abs() < 20.0 && mpos.1 > s.y1 && mpos.1 < s.y2 {
                    s.pluck(20.0);
                }
            }
        }

        // Update strings
        for s in &mut strings {
            s.update(dt);
        }

        // Build magnetic field from vibrating strings
        // Not using diffuse_magnetism since it doesn't exist. We use standard array operations or clear.
        for i in 0..platter.width() {
            for j in 0..platter.height() {
                // simple decay manually since decay_magnetism isn't available
                let current = platter.get_magnetism(i, j);
                if current > 0.01 {
                    platter.magnetize(i, j, -current * 0.05); // primitive decay
                }
            }
        }

        for s in &strings {
            let intensity = s.amplitude as f64 * 0.1;
            let steps = 20;
            let step_size = (s.y2 - s.y1) / steps as f32;
            for i in 0..=steps {
                let y_offset = i as f32 * step_size;
                let ratio = y_offset / (s.y2 - s.y1);
                let shape = (PI * ratio).sin();
                let sx = s.x + s.phase.sin() * s.amplitude * shape;
                let sy = s.y1 + y_offset;

                let gx = (sx / GRID_SCALE) as i32;
                let gy = (sy / GRID_SCALE) as i32;

                if gx >= 0 && gy >= 0 && gx < platter.width() as i32 && gy < platter.height() as i32
                {
                    platter.magnetize(gx as usize, gy as usize, intensity * shape as f64);
                }
            }
        }

        // Update agents
        for agent in &mut agents {
            // Sample magnetic field
            let gx = (agent.x / GRID_SCALE) as i32;
            let gy = (agent.y / GRID_SCALE) as i32;

            let fx;
            let fy;

            let get_mag = |x: i32, y: i32| -> f32 {
                if x >= 0 && y >= 0 && x < platter.width() as i32 && y < platter.height() as i32 {
                    platter.get_magnetism(x as usize, y as usize) as f32
                } else {
                    0.0
                }
            };

            let c = get_mag(gx, gy);
            let r = get_mag(gx + 1, gy);
            let l = get_mag(gx - 1, gy);
            let d = get_mag(gx, gy + 1);
            let u = get_mag(gx, gy - 1);

            // Feed the magnetic sensor value to the VM stack before executing
            // This allows the agent's genetics to interact with the environment
            agent
                .vm
                .stack
                .push(chimera_lang::value::Value::Int((c * 1000.0) as i64));
            agent.vm.step();

            // After stepping, the VM might pop and use the value. We can extract
            // the top of the stack to influence its movement intentionally.
            let mut deliberate_vx = 0.0;
            let mut deliberate_vy = 0.0;
            if let Some(chimera_lang::value::Value::Int(val)) = agent.vm.stack.pop() {
                // Determine direction based on genetic output
                let angle = (val % 360) as f32 * (PI / 180.0);
                deliberate_vx = angle.cos() * 10.0;
                deliberate_vy = angle.sin() * 10.0;
            }

            fx = (r - l) * 100.0;
            fy = (d - u) * 100.0;

            // Random walk from VM "thinking" + deliberate movement from stack
            let think_x = (rand::gen_range(-1.0, 1.0) as f32) * 2.0 + deliberate_vx;
            let think_y = (rand::gen_range(-1.0, 1.0) as f32) * 2.0 + deliberate_vy;

            agent.vx += (fx + think_x) * dt;
            agent.vy += (fy + think_y) * dt;

            agent.vx *= 0.95; // friction
            agent.vy *= 0.95;

            agent.x += agent.vx * dt;
            agent.y += agent.vy * dt;

            // Bounds check
            if agent.x < 0.0 {
                agent.x = 0.0;
                agent.vx *= -0.5;
            }
            if agent.x > WIDTH {
                agent.x = WIDTH;
                agent.vx *= -0.5;
            }
            if agent.y < 0.0 {
                agent.y = 0.0;
                agent.vy *= -0.5;
            }
            if agent.y > HEIGHT {
                agent.y = HEIGHT;
                agent.vy *= -0.5;
            }

            // Agent plucks strings
            for s in &mut strings {
                if (agent.x - s.x).abs() < 5.0 && agent.y > s.y1 && agent.y < s.y2 {
                    s.pluck(2.0);
                    agent.energy -= 1.0;
                }
            }

            agent.age += 1;
        }

        // Remove dead agents
        agents.retain(|a| a.energy > 0.0);

        // Spawn new ones occasionally
        if agents.len() < 50 && rand::gen_range(0, 100) < 5 {
            agents.push(Agent::new(
                rand::gen_range(50.0, WIDTH - 50.0),
                rand::gen_range(50.0, HEIGHT - 50.0),
            ));
        }

        // Draw
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        // Draw magnetic field roughly
        for y in (0..platter.height()).step_by(2) {
            for x in (0..platter.width()).step_by(2) {
                let mag = platter.get_magnetism(x, y) as f32;
                if mag.abs() > 0.1 {
                    let c = if mag > 0.0 {
                        Color::new(1.0, 0.2, 0.2, mag.abs() * 0.5)
                    } else {
                        Color::new(0.2, 0.2, 1.0, mag.abs() * 0.5)
                    };
                    draw_rectangle(
                        x as f32 * GRID_SCALE,
                        y as f32 * GRID_SCALE,
                        GRID_SCALE * 2.0,
                        GRID_SCALE * 2.0,
                        c,
                    );
                }
            }
        }

        // Draw strings
        for s in &strings {
            let segments = 20;
            for i in 0..segments {
                let t1 = i as f32 / segments as f32;
                let t2 = (i + 1) as f32 / segments as f32;

                let y1 = s.y1 + (s.y2 - s.y1) * t1;
                let y2 = s.y1 + (s.y2 - s.y1) * t2;

                let mode = (t1 * PI).sin();
                let mode2 = (t2 * PI).sin();

                let x1 = s.x + s.phase.sin() * s.amplitude * mode;
                let x2 = s.x + s.phase.sin() * s.amplitude * mode2;

                // Color based on amplitude
                let c = Color::new(0.5 + (s.amplitude.abs() / 50.0), 0.5, 1.0, 1.0);

                draw_line(x1, y1, x2, y2, 2.0, c);
            }
        }

        // Draw agents
        for agent in &agents {
            let size = 3.0 + (agent.energy / 100.0) * 2.0;
            let c = Color::new(0.2, 0.8, 0.4, 0.8);
            draw_circle(agent.x, agent.y, size, c);
        }

        draw_text(
            "Chimera Strings: Acoustic Genetic Symbiosis",
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Click: Pluck | Space: Kill Agents",
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
