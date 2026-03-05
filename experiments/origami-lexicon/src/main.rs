use macroquad::prelude::*;

mod mesh;
mod pbd;
mod phonology;

use ::rand::{thread_rng, Rng};
use mesh::Mesh as OrigamiMesh;
use pbd::PbdSystem;
use phonology::{Rule, Word};

fn conf() -> Conf {
    Conf {
        window_title: "🧬 Origami Lexicon".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut system = PbdSystem::new();
    let mut origami = OrigamiMesh::new();

    let initial_text =
        "In the beginning was the Word, and the Word was with God, and the Word was God.";

    // Generate Pattern
    let rows = 12;
    let cols = 12;
    origami.generate_miura_ori(&mut system, rows, cols, initial_text);

    let mut cam_dist = 25.0;
    let mut cam_yaw = 0.5f32;
    let mut cam_pitch = 0.8f32;

    let mut fold_rho = 0.0f32; // 0.0 = flat, 1.0 = folded

    let mut last_mouse_pos = mouse_position();

    // Interaction State
    let mut mutated_indices: Vec<usize> = Vec::new();

    loop {
        let dt = 0.016;

        // Input
        if is_key_down(KeyCode::Up) {
            fold_rho = (fold_rho + 0.01).min(1.0);
        }
        if is_key_down(KeyCode::Down) {
            fold_rho = (fold_rho - 0.01).max(0.0);
        }
        if is_key_pressed(KeyCode::R) {
            system = PbdSystem::new();
            origami.generate_miura_ori(&mut system, rows, cols, initial_text);
            fold_rho = 0.0;
            mutated_indices.clear();
        }

        // Camera
        let mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let delta_x = mouse_pos.0 - last_mouse_pos.0;
            let delta_y = mouse_pos.1 - last_mouse_pos.1;

            cam_yaw -= delta_x * 0.01;
            cam_pitch = (cam_pitch + delta_y * 0.01).clamp(0.1, 1.5);
        }
        last_mouse_pos = mouse_pos;

        let scroll = mouse_wheel().1;
        cam_dist = (cam_dist - scroll * 1.0).clamp(2.0, 80.0);

        // Update Physics
        origami.update_folds(&mut system, fold_rho);
        system.step(dt, 20);

        // Semantic Collisions
        // Only check if folded enough to cause touch, to save perf on flat sheet
        if fold_rho > 0.5 {
            mutated_indices.clear();
            let mut rng = thread_rng();
            let collision_threshold = 0.5; // Tuning needed based on 'a' size (1.0)

            // Calculate centroids first
            let centroids: Vec<Vec3> = origami
                .cells
                .iter()
                .map(|cell| {
                    let p0 = system.particles[cell.indices[0]].pos;
                    let p1 = system.particles[cell.indices[1]].pos;
                    let p2 = system.particles[cell.indices[2]].pos;
                    let p3 = system.particles[cell.indices[3]].pos;
                    (p0 + p1 + p2 + p3) / 4.0
                })
                .collect();

            // Check pairs
            for i in 0..origami.cells.len() {
                for j in (i + 1)..origami.cells.len() {
                    // Check adjacency
                    let ri = i / cols;
                    let ci = i % cols;
                    let rj = j / cols;
                    let cj = j % cols;

                    if (ri as i32 - rj as i32).abs() <= 1 && (ci as i32 - cj as i32).abs() <= 1 {
                        continue; // Skip neighbors
                    }

                    let dist = centroids[i].distance(centroids[j]);
                    if dist < collision_threshold {
                        // Collision!
                        // Mutate content
                        if let (Some(c1), Some(c2)) =
                            (origami.cells[i].content, origami.cells[j].content)
                        {
                            // Convert to word "c1c2"
                            let s = format!("{}{}", c1, c2);
                            let mut w = Word::new(&s);

                            // Apply rules
                            let rules: Vec<Rule> = vec![Rule::GrimmsLaw, Rule::VowelShift];
                            let rule_idx = rng.gen_range(0..rules.len());

                            if rules[rule_idx].apply(&mut w, &mut rng) {
                                // Apply changes back
                                if w.phonemes.len() >= 1 {
                                    origami.cells[i].content = Some(w.phonemes[0].symbol);
                                }
                                if w.phonemes.len() >= 2 {
                                    origami.cells[j].content = Some(w.phonemes[1].symbol);
                                }

                                mutated_indices.push(i);
                                mutated_indices.push(j);
                            }
                        }
                    }
                }
            }
        }

        // Render
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let cam_pos = vec3(
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
        );

        let camera = Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        };
        set_camera(&camera);

        // Draw Mesh
        let light_dir = vec3(0.5, 1.0, 0.5).normalize();

        // We build a macroquad mesh
        let mut mq_mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for (idx, cell) in origami.cells.iter().enumerate() {
            // Each cell has 2 triangles: (0,1,2) and (0,2,3)
            let idxs = [0, 1, 2, 0, 2, 3];

            // Base color
            let mut base_color = if fold_rho > 0.9 {
                Color::new(0.8, 0.3, 0.3, 1.0)
            } else {
                Color::new(0.95, 0.95, 0.9, 1.0)
            };

            if mutated_indices.contains(&idx) {
                base_color = Color::new(0.3, 0.8, 0.3, 1.0); // Green if mutating
            }

            for &k in &idxs {
                let p_idx = cell.indices[k];
                let v = system.particles[p_idx].pos;

                // Normal calc (flat shading per triangle really, but let's approximate)
                // Better: compute per triangle in loop
                let p0 = system.particles[cell.indices[0]].pos;
                let p1 = system.particles[cell.indices[1]].pos;
                let p2 = system.particles[cell.indices[2]].pos;
                let normal = (p1 - p0).cross(p2 - p0).normalize_or_zero();

                let mut intensity = normal.dot(light_dir).abs();
                intensity = 0.2 + 0.8 * intensity;

                let color = Color::new(
                    base_color.r * intensity,
                    base_color.g * intensity,
                    base_color.b * intensity,
                    1.0,
                );

                mq_mesh.vertices.push(Vertex {
                    position: v,
                    uv: Vec2::ZERO,
                    color: color.into(),
                    normal: Vec4::ZERO, // Not used by default material often
                });
            }

            let base_v = mq_mesh.vertices.len() as u16 - 6;
            mq_mesh.indices.push(base_v + 0);
            mq_mesh.indices.push(base_v + 1);
            mq_mesh.indices.push(base_v + 2);
            mq_mesh.indices.push(base_v + 3);
            mq_mesh.indices.push(base_v + 4);
            mq_mesh.indices.push(base_v + 5);
        }
        draw_mesh(&mq_mesh);
        draw_grid(20, 1.0, BLACK, GRAY);

        set_default_camera();

        // Draw Text Overlay
        for cell in &origami.cells {
            if let Some(c) = cell.content {
                let p0 = system.particles[cell.indices[0]].pos;
                let p1 = system.particles[cell.indices[1]].pos;
                let p2 = system.particles[cell.indices[2]].pos;
                let p3 = system.particles[cell.indices[3]].pos;
                let center = (p0 + p1 + p2 + p3) / 4.0;

                // Check visibility: Dot product of normal and view dir
                let normal = (p1 - p0).cross(p2 - p0).normalize_or_zero();
                let view_dir = (cam_pos - center).normalize();

                if normal.dot(view_dir) > 0.0 {
                    let target = vec3(0.0, 0.0, 0.0);
                    let up = vec3(0.0, 1.0, 0.0);
                    let screen_pos = project_to_screen(center, cam_pos, target, up);

                    let x = screen_pos.x;
                    let y = screen_pos.y;

                    // Check depth (z > 0 means in front of camera plane)
                    if screen_pos.z > 0.0 {
                        draw_text(&c.to_string(), x, y, 20.0, WHITE);
                    }
                }
            }
        }

        // UI
        draw_text("🧬 Origami Lexicon", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Fold (Rho): {:.2}", fold_rho),
            20.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text("Use UP/DOWN arrows to fold/unfold.", 20.0, 80.0, 20.0, GRAY);
        draw_text(
            "Folding causes distant words to touch and mutate.",
            20.0,
            100.0,
            20.0,
            GRAY,
        );

        // Show reconstructed text
        let current_text: String = origami.cells.iter().filter_map(|c| c.content).collect();
        draw_text(
            &format!("Text: {:.50}...", current_text),
            20.0,
            screen_height() - 30.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

fn project_to_screen(pos: Vec3, cam_pos: Vec3, target: Vec3, up: Vec3) -> Vec3 {
    let aspect = screen_width() / screen_height();
    let fov = 45.0f32.to_radians();

    let projection = Mat4::perspective_rh_gl(fov, aspect, 0.01, 1000.0);
    let view = Mat4::look_at_rh(cam_pos, target, up);

    let clip_space = projection * view * pos.extend(1.0);

    let ndc = clip_space.truncate() / clip_space.w;

    let x = (ndc.x + 1.0) * 0.5 * screen_width();
    let y = (1.0 - ndc.y) * 0.5 * screen_height();

    // Return pixels x, y and linear depth w (positive = in front)
    vec3(x, y, clip_space.w)
}
