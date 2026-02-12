mod mesh_gen;
mod pbd;

use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use mesh_gen::generate_miura_ori;
use pbd::{Constraint, PbdSystem};
use ::rand::Rng;

const MAX_SPORES: usize = 500;
const INFECTION_RATE: f32 = 0.2;
const SPORE_SPEED: f32 = 0.5;

struct Spore {
    pos: Vec3,
    vel: Vec3,
    target_idx: Option<usize>,
    life: f32,
    color: Color,
}

impl Spore {
    fn new(pos: Vec3) -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            pos,
            vel: vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize() * 0.1,
            target_idx: None,
            life: rng.gen_range(5.0..10.0),
            color: Color::new(rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), rng.gen_range(0.5..1.0), 1.0), // Purple-ish
        }
    }

    fn update(&mut self, dt: f32, system: &PbdSystem) -> bool {
        self.life -= dt;
        if self.life <= 0.0 {
            return false;
        }

        // Seek target
        if let Some(idx) = self.target_idx {
            if idx < system.particles.len() {
                let target_pos = system.particles[idx].pos;
                let dir = (target_pos - self.pos).normalize_or_zero();
                self.vel += dir * dt * 5.0; // Steering
            } else {
                self.target_idx = None;
            }
        } else {
            // Find new target
            if !system.particles.is_empty() {
                let mut rng = ::rand::thread_rng();
                self.target_idx = Some(rng.gen_range(0..system.particles.len()));
            }
        }

        // Apply velocity
        self.vel = self.vel.clamp_length_max(SPORE_SPEED);
        self.pos += self.vel;

        true
    }
}

#[macroquad::main("Origami Spores")]
async fn main() {
    let mesh_data = generate_miura_ori(10, 10);
    // Destructure to avoid partial move issues
    let mesh_gen::MeshData {
        mut system,
        indices,
        actuators: _,
    } = mesh_data;

    let mut infection_levels = vec![0.0f32; system.particles.len()];
    let mut spores: Vec<Spore> = Vec::new();

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 25.0;

    let mut last_mouse_pos = mouse_position();

    loop {
        let dt = 0.016; // Fixed step for consistency

        // Input
        let mouse_pos = mouse_position();
        let delta = vec2(
            mouse_pos.0 - last_mouse_pos.0,
            mouse_pos.1 - last_mouse_pos.1,
        );
        last_mouse_pos = mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        // Spawning
        if spores.len() < MAX_SPORES {
            if ::rand::thread_rng().gen_bool(0.1) {
                let spawn_pos = vec3(
                    ::rand::thread_rng().gen_range(-20.0..20.0),
                    ::rand::thread_rng().gen_range(5.0..20.0),
                    ::rand::thread_rng().gen_range(-20.0..20.0),
                );
                spores.push(Spore::new(spawn_pos));
            }
        }

        // Update Spores
        let mut landed_spores = Vec::new();
        spores.retain_mut(|spore| {
            if !spore.update(dt, &system) {
                return false;
            }

            // Check collision with target
            if let Some(idx) = spore.target_idx {
                 if idx < system.particles.len() {
                    let target_pos = system.particles[idx].pos;
                    if spore.pos.distance_squared(target_pos) < 0.5 {
                        landed_spores.push(idx);
                        return false; // Absorb spore
                    }
                 }
            }
            true
        });

        // Apply Infection
        for idx in landed_spores {
            if idx < infection_levels.len() {
                infection_levels[idx] = (infection_levels[idx] + INFECTION_RATE).min(1.0);
            }
        }

        // Apply Infection Effects on Constraints
        // 1. Weaken structural constraints connected to infected particles (paper gets soggy)
        // 2. Actuate folds based on infection (crumpling)
        for constraint in &mut system.constraints {
            match constraint {
                Constraint::Distance { p1, p2, ref mut stiffness, .. } => {
                     let inf1 = infection_levels[*p1];
                     let inf2 = infection_levels[*p2];
                     if inf1 > 0.0 || inf2 > 0.0 {
                         // Weaken stiffness
                         *stiffness = (1.0 - (inf1 + inf2) * 0.4).max(0.1);
                     }
                }
                Constraint::Actuator { p1, p2, ref mut factor, .. } => {
                     let inf1 = infection_levels[*p1];
                     let inf2 = infection_levels[*p2];
                     if inf1 > 0.0 || inf2 > 0.0 {
                         // Force fold: Close the crease (factor -> 0.0)
                         // The more infected, the more it folds
                         let target_fold = 0.0;
                         let strength = (inf1 + inf2) * 0.5;
                         *factor = *factor * (1.0 - strength) + target_fold * strength;
                     }
                }
                _ => {}
            }
        }

        // Physics Step
        system.step(dt, 10);

        // Rendering
        clear_background(BLACK);

        let cam_pos = vec3(
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Spores
        for spore in &spores {
            draw_line_3d(spore.pos, spore.pos + spore.vel * 2.0, spore.color);
        }

        // Draw Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in (0..indices.len()).step_by(3) {
            let idx0 = indices[i] as usize;
            let idx1 = indices[i + 1] as usize;
            let idx2 = indices[i + 2] as usize;

            let v0 = system.particles[idx0].pos;
            let v1 = system.particles[idx1].pos;
            let v2 = system.particles[idx2].pos;

            // Flat shading
            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let light_dir = vec3(0.5, 1.0, 0.5).normalize();
            let intensity = normal.dot(light_dir).abs() * 0.7 + 0.3;

            // Color based on infection
            // Average infection of triangle
            let avg_inf = (infection_levels[idx0] + infection_levels[idx1] + infection_levels[idx2]) / 3.0;

            // Base Blue -> Infected Green/Purple
            let r = avg_inf * 0.8;
            let g = avg_inf * 0.2;
            let b = (1.0 - avg_inf) * 0.8;

            let color = Color::new(r * intensity, g * intensity, b * intensity, 1.0);

            let start_idx = mesh.vertices.len() as u16;

            let color_bytes: [u8; 4] = color.into();
            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);
            mesh.vertices.push(Vertex {
                position: v0,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v1,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v2,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);
        }

        draw_mesh(&mesh);

        // Draw Wireframe for debugging structure
        /*
        for i in (0..indices.len()).step_by(3) {
            let v0 = system.particles[indices[i] as usize].pos;
            let v1 = system.particles[indices[i+1] as usize].pos;
            let v2 = system.particles[indices[i+2] as usize].pos;
            draw_line_3d(v0, v1, GRAY);
            draw_line_3d(v1, v2, GRAY);
            draw_line_3d(v2, v0, GRAY);
        }
        */

        set_default_camera();

        draw_text("Origami Spores", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Spores: {}", spores.len()), 10.0, 50.0, 20.0, WHITE);
        draw_text("Spores infect mesh -> Mesh crumples", 10.0, 70.0, 20.0, GRAY);

        next_frame().await
    }
}
