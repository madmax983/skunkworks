use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};

const GRID_W: usize = 300;
const GRID_H: usize = 300;
const GRID_SCALE: f32 = 2.0;

struct PheromoneGrid {
    width: usize,
    height: usize,
    cells: Vec<f32>,
}

impl PheromoneGrid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            cells: vec![0.0; w * h],
        }
    }

    fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            0.0
        }
    }

    fn add(&mut self, x: usize, y: usize, val: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.cells[idx] = (self.cells[idx] + val).min(1.0);
        }
    }

    fn decay(&mut self, rate: f32) {
        for c in &mut self.cells {
            *c *= rate;
        }
    }

    // Smooth diffusion
    fn diffuse(&mut self, diff: &mut [f32]) {
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = y * self.width + x;
                let sum = self.cells[idx - self.width]
                    + self.cells[idx + self.width]
                    + self.cells[idx - 1]
                    + self.cells[idx + 1];
                let cur = self.cells[idx];
                diff[idx] = cur + (sum / 4.0 - cur) * 0.2; // slight diffusion
            }
        }
        self.cells.copy_from_slice(diff);
    }
}

// Slime mold sensor logic
struct AgentParams {
    sensor_angle: f32,
    sensor_dist: f32,
    rotation_angle: f32,
    speed: f32,
}

// Each particle in the PBD system has an associated "head" angle
struct Head {
    angle: f32,
}

#[macroquad::main("Myco-Tissue")]
async fn main() {
    let mut grid = PheromoneGrid::new(GRID_W, GRID_H);
    let mut diff_buffer = vec![0.0; GRID_W * GRID_H];

    let mut system = PbdSystem::new();
    let mut heads = Vec::new();

    let cx = GRID_W as f32 / 2.0;
    let cy = GRID_H as f32 / 2.0;

    // Create a circular tissue of agents
    let num_agents = 100;
    let radius = 20.0;

    for i in 0..num_agents {
        let angle = (i as f32 / num_agents as f32) * std::f32::consts::TAU;
        let x = cx + angle.cos() * radius;
        let y = cy + angle.sin() * radius;

        let _p = system.add_particle(vec3(x, y, 0.0), 1.0);
        heads.push(Head { angle });
    }

    // Add perimeter constraints
    for i in 0..num_agents {
        let next = (i + 1) % num_agents;
        system.add_distance_constraint(i, next, 0.5);
    }

    // Add internal bracing to keep shape roughly circular, but squishy
    let center_idx = system.particles.len();
    system.add_particle(vec3(cx, cy, 0.0), 1.0); // Center point
    heads.push(Head { angle: 0.0 }); // Dummy head

    for i in 0..num_agents {
        system.add_distance_constraint(i, center_idx, 0.1);
    }

    // Food sources to attract them
    let foods = vec![
        vec2(cx + 80.0, cy + 20.0),
        vec2(cx - 50.0, cy - 80.0),
        vec2(cx + 10.0, cy + 100.0),
    ];

    let params = AgentParams {
        sensor_angle: std::f32::consts::PI / 4.0,
        sensor_dist: 15.0,
        rotation_angle: std::f32::consts::PI / 8.0,
        speed: 15.0,
    };

    let mut image_buffer = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLACK);
    let texture = Texture2D::from_image(&image_buffer);
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time().min(0.033);

        if is_key_pressed(KeyCode::Space) {
            // Reset logic
            grid = PheromoneGrid::new(GRID_W, GRID_H);
            for (i, p) in system.particles.iter_mut().enumerate().take(num_agents) {
                let angle = (i as f32 / num_agents as f32) * std::f32::consts::TAU;
                p.pos.x = cx + angle.cos() * radius;
                p.pos.y = cy + angle.sin() * radius;
                p.prev_pos = p.pos;
                heads[i].angle = angle;
            }
            system.particles[center_idx].pos = vec3(cx, cy, 0.0);
            system.particles[center_idx].prev_pos = vec3(cx, cy, 0.0);
        }

        // Draw food pheromones
        for f in &foods {
            grid.add(f.x as usize, f.y as usize, 1.0);
        }

        // 1. Agent Foraging (Chemotaxis -> Physics Velocity)
        for (i, head) in heads.iter_mut().enumerate().take(num_agents) {
            let p = &system.particles[i];
            let pos = p.pos;

            // Sense Forward, Left, Right
            let sense = |angle: f32| -> f32 {
                let sx = (pos.x + angle.cos() * params.sensor_dist) as i32;
                let sy = (pos.y + angle.sin() * params.sensor_dist) as i32;

                let mut sum = 0.0;
                // Small box sample
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if sx + dx >= 0
                            && sx + dx < GRID_W as i32
                            && sy + dy >= 0
                            && sy + dy < GRID_H as i32
                        {
                            sum += grid.get((sx + dx) as usize, (sy + dy) as usize);
                        }
                    }
                }
                sum
            };

            let fwd = sense(head.angle);
            let left = sense(head.angle - params.sensor_angle);
            let right = sense(head.angle + params.sensor_angle);

            // Rotate based on gradient
            if fwd > left && fwd > right {
                // Stay course
            } else if left > right {
                head.angle -= params.rotation_angle * dt * 60.0;
            } else if right > left {
                head.angle += params.rotation_angle * dt * 60.0;
            } else {
                // Random wander
                head.angle += (macroquad::rand::gen_range(-1.0f32, 1.0f32)) * 0.1;
            }

            // Apply intent as a force to the particle's velocity
            let intent = vec3(head.angle.cos(), head.angle.sin(), 0.0) * params.speed * dt;
            system.particles[i].pos += intent;
        }

        // Add damping and apply PBD step
        for p in &mut system.particles {
            // Very light damping to prevent explosions
            let v = p.pos - p.prev_pos;
            p.pos = p.prev_pos + v * 0.99;
        }

        system.step(dt, 4); // 4 solver iterations

        // Keep particles in bounds
        for p in &mut system.particles {
            p.pos.x = p.pos.x.clamp(2.0, GRID_W as f32 - 2.0);
            p.pos.y = p.pos.y.clamp(2.0, GRID_H as f32 - 2.0);
        }

        // Deposit pheromones at new positions
        for i in 0..num_agents {
            let pos = system.particles[i].pos;
            grid.add(pos.x as usize, pos.y as usize, 0.8);
        }

        grid.diffuse(&mut diff_buffer);
        grid.decay(0.98);

        // Render Background (Pheromones)
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let v = grid.get(x, y);
                // Slime mold yellow-green
                let color = Color::new(v * 0.8, v, v * 0.2, 1.0);
                image_buffer.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image_buffer);

        clear_background(BLACK);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GRID_W as f32 * GRID_SCALE, GRID_H as f32 * GRID_SCALE)),
                ..Default::default()
            },
        );

        // Render Soft Body
        for c in &system.constraints {
            if let Constraint::Distance { p1, p2, .. } = c {
                let pos1 = system.particles[*p1].pos * GRID_SCALE;
                let pos2 = system.particles[*p2].pos * GRID_SCALE;
                // Draw internal struts dark, perimeter bright
                let color = if *p1 == center_idx || *p2 == center_idx {
                    Color::new(0.5, 0.5, 0.0, 0.3)
                } else {
                    YELLOW
                };
                draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, color);
            }
        }

        // Draw foods
        for f in &foods {
            draw_circle(f.x * GRID_SCALE, f.y * GRID_SCALE, 5.0, RED);
        }

        draw_text(
            "Myco-Tissue: Elastic Pheromone Network",
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text("Space to Reset", 10.0, 60.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
