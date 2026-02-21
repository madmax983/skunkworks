mod math4d;

use chimera_lang::prelude::*;
use macroquad::prelude::*;
use math4d::{generate_tesseract_base, Vec4};

struct HyperAgent {
    vm: ChimeraVM,
    color: Color,
}

impl HyperAgent {
    fn new() -> Self {
        // Simple DNA: Photosynthesize to gain energy, Loop to stay alive
        let genes = vec![
            Gene {
                op: OpCode::Photosynthesize,
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
        let mut vm = ChimeraVM::new(dna);
        // Give some initial energy
        vm.energy = 100;

        Self { vm, color: WHITE }
    }

    fn update(&mut self) {
        // Step the VM
        if self.vm.energy > 0 {
            self.vm.step();
        }

        // Update color based on energy
        // High energy -> Green
        // Low energy -> Red
        // Dead -> Gray
        if self.vm.energy == 0 {
            self.color = GRAY;
        } else {
            let energy_factor = (self.vm.energy as f32 / 200.0).clamp(0.0, 1.0);
            self.color = Color::new(1.0 - energy_factor, energy_factor, 0.2, 1.0);
        }
    }
}

#[macroquad::main("Chimera Tesseract")]
async fn main() {
    let (base_verts, edges) = generate_tesseract_base();

    // Create 16 agents, one for each vertex
    let mut agents: Vec<HyperAgent> = (0..16).map(|_| HyperAgent::new()).collect();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 6.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        let dt = get_frame_time();

        // Update Agents
        let mut total_energy = 0;
        for agent in &mut agents {
            agent.update();
            total_energy += agent.vm.energy;
        }

        // Calculate colony metrics
        let avg_energy = total_energy as f32 / agents.len() as f32;

        // Rotation speed based on colony energy
        // More energy = Faster rotation (Hyper-metabolism)
        let rotation_speed = 0.2 + (avg_energy / 500.0);

        angle_xw += dt * rotation_speed;
        angle_yw += dt * rotation_speed * 0.7;
        angle_zw += dt * rotation_speed * 0.3;

        // Input Camera
        if is_key_down(KeyCode::Left) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 2.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 2.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 5.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0 * dt;
        }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Transform Vertices
        // W-axis scale: Breathing effect based on time
        let sw = (get_time() as f32).sin() * 0.1;

        let transform = |v: Vec4| -> Vec3 {
            let mut v = v.scale_dim(1.0, 1.0, 1.0, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            v.project_to_3d(3.0)
        };

        // Draw Edges
        for &(i, j) in &edges {
            let v1 = base_verts[i];
            let v2 = base_verts[j];
            let p1 = transform(v1);
            let p2 = transform(v2);

            // Edge color is average of connected agents
            let c1 = agents[i].color;
            let c2 = agents[j].color;
            let edge_color = Color::new(
                (c1.r + c2.r) * 0.5,
                (c1.g + c2.g) * 0.5,
                (c1.b + c2.b) * 0.5,
                0.5, // Transparent
            );

            draw_line_3d(p1, p2, edge_color);
        }

        // Draw Vertices (Agents)
        for (i, v) in base_verts.iter().enumerate() {
            let p = transform(*v);
            let agent = &agents[i];

            // Sphere size based on energy
            // Dead agents are small specks
            let size = if agent.vm.energy == 0 {
                0.02
            } else {
                0.05 + (agent.vm.energy as f32 / 1000.0).clamp(0.0, 0.2)
            };

            draw_sphere(p, size, None, agent.color);
        }

        set_default_camera();

        // HUD
        draw_text("Chimera Tesseract", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Total Energy: {}", total_energy),
            10.0,
            50.0,
            20.0,
            GREEN,
        );
        draw_text(
            &format!("Avg Energy: {:.1}", avg_energy),
            10.0,
            70.0,
            20.0,
            YELLOW,
        );
        draw_text(
            &format!("Rotation Speed: {:.2}", rotation_speed),
            10.0,
            90.0,
            20.0,
            BLUE,
        );
        draw_text(
            "Arrows/WASD to Rotate Camera",
            10.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
