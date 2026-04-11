// LINEAGE:
// - Parent A: crates/flocking (Craig Reynolds' Boids algorithm, providing continuous 2D spatial swarming mechanics)
// - Parent B: crates/origami (Miura-ori soft body mesh generation and constraint-based physics via Position Based Dynamics)
// - Novel Trait: Flocking density dynamically actuates 3D structural constraints on the origami mesh.

mod mesh_gen;
mod pbd;

use ::rand::Rng;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use mesh_gen::generate_miura_ori;
use pbd::{Constraint, PbdSystem};

const MAX_BOIDS: usize = 200;
const BOID_SPEED: f64 = 2.0;
const BOID_SCALE: f32 = 0.5;

struct Boid {
    pos: Vec2,
    vel: Vec2,
    color: Color,
}

impl Boid {
    fn new(pos: Vec2) -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            pos,
            vel: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
                * BOID_SPEED,
            color: Color::new(
                rng.gen_range(0.2..0.8),
                rng.gen_range(0.6..1.0),
                rng.gen_range(0.8..1.0),
                1.0,
            ),
        }
    }
}

#[macroquad::main("Flock Origami")]
async fn main() {
    let rows = 15;
    let cols = 15;
    let mesh_data = generate_miura_ori(rows, cols);
    let mut system = mesh_data.system;
    let indices = mesh_data.indices;

    let mut boids: Vec<Boid> = Vec::new();
    for _ in 0..MAX_BOIDS {
        let pos = Vec2::new(
            ::rand::thread_rng().gen_range(-20.0..20.0),
            ::rand::thread_rng().gen_range(-20.0..20.0),
        );
        boids.push(Boid::new(pos));
    }

    let flocking_params = FlockingParams {
        view_radius: 5.0,
        separation_radius: 1.5,
        max_speed: BOID_SPEED,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 40.0;

    let mut last_mouse_pos = mouse_position();

    loop {
        let dt = 0.016;

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

        // Flocking Update
        let positions: Vec<Vec2> = boids.iter().map(|b| b.pos).collect();
        let velocities: Vec<Vec2> = boids.iter().map(|b| b.vel).collect();

        for (i, boid) in boids.iter_mut().enumerate() {
            let force = compute_force(&positions, &velocities, i, &flocking_params);
            boid.vel += force;
            let mag = boid.vel.magnitude();
            if mag > BOID_SPEED {
                boid.vel = (boid.vel / mag) * BOID_SPEED;
            }
            boid.pos += boid.vel * (dt as f64) * 10.0;

            // Bounds wrapping
            if boid.pos.x > 25.0 {
                boid.pos.x = -25.0;
            }
            if boid.pos.x < -25.0 {
                boid.pos.x = 25.0;
            }
            if boid.pos.y > 25.0 {
                boid.pos.y = -25.0;
            }
            if boid.pos.y < -25.0 {
                boid.pos.y = 25.0;
            }
        }

        // Apply Boid density to Mesh Constraints
        // Calculate spatial density of boids
        let mut density = vec![0.0f32; system.particles.len()];
        for boid in &boids {
            let bpos = vec3(boid.pos.x as f32, 0.0, boid.pos.y as f32);
            for (idx, particle) in system.particles.iter().enumerate() {
                // Ignore Y for distance to project boids onto paper
                let dist_sq = (particle.pos.x - bpos.x).powi(2) + (particle.pos.z - bpos.z).powi(2);
                if dist_sq < 25.0 {
                    density[idx] += 1.0 / (1.0 + dist_sq);
                }
            }
        }

        for constraint in &mut system.constraints {
            match constraint {
                Constraint::Distance {
                    p1,
                    p2,
                    ref mut stiffness,
                    ..
                } => {
                    let d1 = density[*p1];
                    let d2 = density[*p2];
                    if d1 > 0.0 || d2 > 0.0 {
                        // Flocking density stiffens the paper locally
                        *stiffness = (0.5 + (d1 + d2) * 0.1).min(1.0);
                    } else {
                        *stiffness = 0.5; // default
                    }
                }
                Constraint::Actuator {
                    p1,
                    p2,
                    ref mut factor,
                    ..
                } => {
                    let d1 = density[*p1];
                    let d2 = density[*p2];
                    if d1 > 0.0 || d2 > 0.0 {
                        // Force fold: Close the crease (factor -> 0.0) based on flock density
                        let target_fold = 0.0;
                        let strength = ((d1 + d2) * 0.05).min(1.0);
                        *factor = *factor * (1.0 - strength) + target_fold * strength;
                    } else {
                        // Relax towards open
                        *factor = *factor * 0.95 + 1.0 * 0.05;
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

        // Draw Boids
        for boid in &boids {
            // Project boid onto mesh surface approximately (find nearest particle height)
            let mut nearest_y = 0.0;
            let mut min_dist = f32::MAX;
            for particle in &system.particles {
                let dist_sq = (particle.pos.x - boid.pos.x as f32).powi(2)
                    + (particle.pos.z - boid.pos.y as f32).powi(2);
                if dist_sq < min_dist {
                    min_dist = dist_sq;
                    nearest_y = particle.pos.y;
                }
            }

            let pos3d = vec3(boid.pos.x as f32, nearest_y + 1.0, boid.pos.y as f32);
            let vel3d = vec3(boid.vel.x as f32, 0.0, boid.vel.y as f32).normalize_or_zero();

            draw_line_3d(pos3d, pos3d - vel3d * BOID_SCALE, boid.color);
            draw_cube(pos3d, vec3(0.2, 0.2, 0.2), None, boid.color);
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

            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let light_dir = vec3(0.5, 1.0, 0.5).normalize();
            let intensity = normal.dot(light_dir).abs() * 0.7 + 0.3;

            let avg_d = (density[idx0] + density[idx1] + density[idx2]) / 3.0;

            let r = (0.2 + avg_d * 0.5).min(1.0);
            let g = (0.2 + avg_d * 0.2).min(1.0);
            let b = (0.8 - avg_d * 0.4).max(0.0);

            let color = Color::new(r * intensity, g * intensity, b * intensity, 1.0);

            let start_idx = mesh.vertices.len() as u16;

            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);

            mesh.vertices.push(Vertex {
                position: v0,
                uv: vec2(0., 0.),
                color: color.into(),
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v1,
                uv: vec2(0., 0.),
                color: color.into(),
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v2,
                uv: vec2(0., 0.),
                color: color.into(),
                normal: normal_v4,
            });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);
        }

        draw_mesh(&mesh);

        set_default_camera();

        draw_text("Flock Origami", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Boids: {}", boids.len()), 10.0, 50.0, 20.0, WHITE);
        draw_text(
            "Boid density actively crumples the mesh",
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
