mod chem_sim;
mod organism;

use ::rand::Rng;
use chem_sim::ChemicalState;
use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::*;
use organism::Organism;

const GRID_SIZE: usize = 200;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chimera Terra".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

struct Terrain {
    mesh: Mesh,
    grid_width: usize,
    grid_height: usize,
}

impl Terrain {
    fn new(width: usize, height: usize) -> Self {
        let mut vertices = Vec::with_capacity(width * height);
        let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);

        for y in 0..height {
            for x in 0..width {
                vertices.push(Vertex {
                    position: vec3(x as f32, 0.0, y as f32),
                    uv: vec2(x as f32 / width as f32, y as f32 / height as f32),
                    color: WHITE.into(),
                    normal: vec4(0.0, 1.0, 0.0, 0.0),
                });
            }
        }

        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let tl = (y * width + x) as u16;
                let tr = (y * width + x + 1) as u16;
                let bl = ((y + 1) * width + x) as u16;
                let br = ((y + 1) * width + x + 1) as u16;

                indices.push(tl);
                indices.push(bl);
                indices.push(tr);

                indices.push(tr);
                indices.push(bl);
                indices.push(br);
            }
        }

        Self {
            mesh: Mesh {
                vertices,
                indices,
                texture: None,
            },
            grid_width: width,
            grid_height: height,
        }
    }

    fn update_from_sim(&mut self, sim: &ChemicalState) {
        // Map u/v to height/color

        for (i, v) in self.mesh.vertices.iter_mut().enumerate() {
            let x = i % self.grid_width;
            let y = i / self.grid_width;

            let _u_val = sim.get_u(x, y);
            let v_val = sim.get_v(x, y);

            // Height: V makes mountains.
            // Scale: 0.0 -> 0.0, 1.0 -> 50.0
            let height = v_val * 50.0;

            v.position.y = height;

            // Color ramp
            v.color = if v_val < 0.1 {
                // Deep Blue
                let t = v_val / 0.1;
                Color::new(0.0, 0.2 + t * 0.1, 0.5 + t * 0.5, 1.0)
            } else if v_val < 0.25 {
                // Sand
                Color::new(0.9, 0.8, 0.5, 1.0)
            } else if v_val < 0.4 {
                // Green
                let t = (v_val - 0.25) / 0.15;
                Color::new(0.1 + t * 0.1, 0.6 + t * 0.2, 0.1, 1.0)
            } else {
                // Coral/Pink/Purple
                let t = (v_val - 0.4) / 0.6;
                Color::new(0.8 + t * 0.2, 0.2 + t * 0.1, 0.4 + t * 0.6, 1.0)
            }
            .into();
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = ChemicalState::new(GRID_SIZE, GRID_SIZE);
    sim.seed_noise();

    let mut terrain = Terrain::new(GRID_SIZE, GRID_SIZE);

    // Spawn Organisms
    let mut organisms = Vec::new();
    let mut rng = ::rand::thread_rng();
    for i in 0..50 {
        let x = rng.gen_range(0..GRID_SIZE);
        let y = rng.gen_range(0..GRID_SIZE);
        organisms.push(Organism::new(x, y, i));
    }

    // Camera state
    let mut cam_pos = vec3(GRID_SIZE as f32 / 2.0, 100.0, GRID_SIZE as f32 * 1.5);
    let mut cam_yaw: f32 = -1.57;
    let mut cam_pitch: f32 = -0.5;

    let mut feed = 0.055;
    let mut kill = 0.062;
    let steps_per_frame = 4;

    loop {
        // Update Simulation
        for _ in 0..steps_per_frame {
            sim.update(1.0, feed, kill);

            // Update Organisms (Sequential for simplicity)
            for org in &mut organisms {
                org.tick(&mut sim);
            }

            // Remove dead organisms
            organisms.retain(|o| o.energy > 0.0);

            // Repopulate if low (maintain population)
            if organisms.len() < 20 {
                let x = rng.gen_range(0..GRID_SIZE);
                let y = rng.gen_range(0..GRID_SIZE);
                organisms.push(Organism::new(x, y, rng.gen()));
            }
        }

        // Update Mesh
        terrain.update_from_sim(&sim);

        // Input Handling (Camera)
        let dt = get_frame_time();
        let speed = 50.0;
        let rot_speed = 1.0;

        if is_key_down(KeyCode::W) {
            cam_pos.x += cam_yaw.cos() * speed * dt;
            cam_pos.z += cam_yaw.sin() * speed * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_pos.x -= cam_yaw.cos() * speed * dt;
            cam_pos.z -= cam_yaw.sin() * speed * dt;
        }
        if is_key_down(KeyCode::A) {
            cam_pos.x += (cam_yaw - std::f32::consts::FRAC_PI_2).cos() * speed * dt;
            cam_pos.z += (cam_yaw - std::f32::consts::FRAC_PI_2).sin() * speed * dt;
        }
        if is_key_down(KeyCode::D) {
            cam_pos.x += (cam_yaw + std::f32::consts::FRAC_PI_2).cos() * speed * dt;
            cam_pos.z += (cam_yaw + std::f32::consts::FRAC_PI_2).sin() * speed * dt;
        }
        if is_key_down(KeyCode::Q) {
            cam_pos.y -= speed * dt;
        }
        if is_key_down(KeyCode::E) {
            cam_pos.y += speed * dt;
        }

        if is_key_down(KeyCode::Left) {
            cam_yaw -= rot_speed * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw += rot_speed * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch += rot_speed * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch -= rot_speed * dt;
        }

        // Interaction: Paint at Crosshair
        if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) {
            let forward = vec3(
                cam_yaw.cos() * cam_pitch.cos(),
                cam_pitch.sin(),
                cam_yaw.sin() * cam_pitch.cos(),
            )
            .normalize();

            if forward.y.abs() > 0.001 {
                let t = -cam_pos.y / forward.y;
                if t > 0.0 {
                    let hit_pos = cam_pos + forward * t;
                    let hx = hit_pos.x.round() as isize;
                    let hz = hit_pos.z.round() as isize;

                    if hx >= 0 && hx < GRID_SIZE as isize && hz >= 0 && hz < GRID_SIZE as isize {
                        sim.add_chemical_blob(hx as usize, hz as usize, 5.0, 0.5);
                    }
                }
            }
        }

        clear_background(BLACK);

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: cam_pos
                + vec3(
                    cam_yaw.cos() * cam_pitch.cos(),
                    cam_pitch.sin(),
                    cam_yaw.sin() * cam_pitch.cos(),
                ),
            ..Default::default()
        });

        draw_grid(GRID_SIZE as u32, 1.0, BLACK, GRAY);

        draw_mesh(&terrain.mesh);

        // Draw Organisms
        for org in &organisms {
            // Get height at org position
            let height = sim.get_v(org.x, org.y) * 50.0;
            let pos = vec3(org.x as f32, height + 1.0, org.y as f32);

            // Pulse size based on energy
            let size = 0.5 + (org.energy / 100.0) * 0.5;

            draw_sphere(pos, size, None, RED);
        }

        set_default_camera();

        // UI
        draw_text("Chimera Terra", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 60.0, 20.0, WHITE);
        draw_text(
            &format!("Organisms: {}", organisms.len()),
            20.0,
            90.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Feed: {:.4} (J/K)", feed),
            20.0,
            110.0,
            20.0,
            WHITE,
        );
        draw_text(
            "WASD: Move, Arrows: Look, Q/E: Up/Down",
            20.0,
            140.0,
            20.0,
            WHITE,
        );

        // Logic for Parameters
        if is_key_down(KeyCode::J) {
            feed -= 0.0001;
        }
        if is_key_down(KeyCode::K) {
            feed += 0.0001;
        }
        if is_key_down(KeyCode::U) {
            kill -= 0.0001;
        }
        if is_key_down(KeyCode::I) {
            kill += 0.0001;
        }

        if is_key_pressed(KeyCode::R) {
            sim = ChemicalState::new(GRID_SIZE, GRID_SIZE);
            sim.seed_noise();
            organisms.clear();
            for i in 0..50 {
                let x = rng.gen_range(0..GRID_SIZE);
                let y = rng.gen_range(0..GRID_SIZE);
                organisms.push(Organism::new(x, y, i));
            }
        }

        next_frame().await
    }
}
