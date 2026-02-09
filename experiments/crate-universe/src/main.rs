use macroquad::prelude::*;
use cargo_metadata::MetadataCommand;
use physics::{Body, Universe};
use ::rand::Rng; // Explicitly use external crate
use std::collections::VecDeque;

pub mod physics;

#[cfg(test)]
mod physics_test;

fn load_dependencies() -> Universe {
    let mut universe = Universe::new();
    // Reduce G for stability in this specific simulation scale
    universe.G = 50.0;

    // 1. Get Metadata
    let metadata = match MetadataCommand::new().exec() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to load metadata: {}", e);
            // Fallback: Create a dummy universe
            let mut rng = ::rand::thread_rng();
            for i in 0..10 {
                universe.add_body(Body {
                    pos: vec2(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0)),
                    vel: vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                    mass: rng.gen_range(10.0..50.0),
                    radius: rng.gen_range(5.0..15.0),
                    name: format!("Dummy {}", i),
                    history: VecDeque::new(),
                    parent_id: None,
                });
            }
            return universe;
        }
    };

    let root = metadata.root_package().expect("No root package found");

    // Map: Package ID -> Body Index
    let mut pkg_map = std::collections::HashMap::new();

    // Add Root
    let root_idx = universe.bodies.len();
    universe.add_body(Body {
        pos: Vec2::ZERO,
        vel: Vec2::ZERO,
        mass: 1000.0,
        radius: 20.0,
        name: root.name.clone(),
        history: VecDeque::new(),
        parent_id: None,
    });
    pkg_map.insert(root.id.clone(), root_idx);

    let mut rng = ::rand::thread_rng();

    // Use `resolve` graph if available.
    let resolve = metadata.resolve.as_ref().expect("No resolve graph found");

    // Breadth-First Search
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((root.id.clone(), root_idx, 0)); // (PackageId, ParentIdx, Depth)

    let mut visited = std::collections::HashSet::new();
    visited.insert(root.id.clone());

    while let Some((pkg_id, parent_idx, depth)) = queue.pop_front() {
        // Find children in resolve graph
        if let Some(node) = resolve.nodes.iter().find(|n| n.id == pkg_id) {
            for dep in &node.dependencies {
                if !visited.contains(dep) {
                    visited.insert(dep.clone());

                    // Find package info
                    if let Some(pkg) = metadata.packages.iter().find(|p| p.id == *dep) {
                        let radius_base = 150.0 * (depth as f32 + 1.0);
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

                        let parent_pos = universe.bodies[parent_idx].pos;
                        let pos = parent_pos + vec2(radius_base * angle.cos(), radius_base * angle.sin());

                        // Orbital Velocity v = sqrt(GM/r) relative to parent
                        let r_vec = pos - parent_pos;
                        let r = r_vec.length();
                        let parent_mass = universe.bodies[parent_idx].mass;
                        let v_mag = (universe.G * parent_mass / r).sqrt();

                        // Tangent direction: (-y, x)
                        let tangent = vec2(-r_vec.y, r_vec.x).normalize();
                        let vel = universe.bodies[parent_idx].vel + tangent * v_mag;

                        let body_idx = universe.bodies.len();
                        universe.add_body(Body {
                            pos,
                            vel,
                            mass: 10.0 + (pkg.targets.len() as f32) * 5.0, // Mass based on targets?
                            radius: 5.0 + (pkg.name.len() as f32 * 0.5).min(10.0),
                            name: pkg.name.clone(),
                            history: VecDeque::new(),
                            parent_id: Some(parent_idx),
                        });
                        pkg_map.insert(pkg.id.clone(), body_idx);

                        // Only traverse deeper if depth < 2 to avoid explosion
                        if depth < 2 {
                            queue.push_back((dep.clone(), body_idx, depth + 1));
                        }
                    }
                }
            }
        }
    }

    println!("Loaded {} bodies", universe.bodies.len());
    universe
}

#[macroquad::main("Crate Universe")]
async fn main() {
    let mut universe = load_dependencies();

    let mut cam_pos = Vec2::ZERO;
    let mut cam_zoom = 0.002; // Start zoomed out to see ~1000 units
    let mut show_names = true;
    let mut show_trails = true;

    loop {
        // Input
        // Pan speed depends on zoom (zoomed out = faster pan)
        let pan_speed = 0.02 / cam_zoom;
        if is_key_down(KeyCode::Right) { cam_pos.x += pan_speed; }
        if is_key_down(KeyCode::Left) { cam_pos.x -= pan_speed; }
        if is_key_down(KeyCode::Up) { cam_pos.y += pan_speed; }
        if is_key_down(KeyCode::Down) { cam_pos.y -= pan_speed; }

        if is_key_pressed(KeyCode::N) { show_names = !show_names; }
        if is_key_pressed(KeyCode::T) { show_trails = !show_trails; }

        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            cam_zoom *= if wheel_y > 0.0 { 1.1 } else { 0.9 };
        }

        // Physics
        universe.step(get_frame_time().min(0.05)); // Cap dt

        // Render
        clear_background(BLACK);

        // Standard Camera2D Setup
        let aspect = screen_width() / screen_height();
        // Adjust zoom so that cam_zoom=1.0 maps -1..1 to vertical height
        let zoom_vec = vec2(cam_zoom / aspect, cam_zoom);

        set_camera(&Camera2D {
            target: cam_pos,
            zoom: zoom_vec,
            ..Default::default()
        });


        // Draw Trails
        if show_trails {
            for body in &universe.bodies {
                if body.history.len() > 1 {
                    for i in 0..body.history.len() - 1 {
                        draw_line(
                            body.history[i].x, body.history[i].y,
                            body.history[i+1].x, body.history[i+1].y,
                            1.0 / cam_zoom, // Thin line, scale with zoom
                            Color::new(0.5, 0.5, 0.5, 0.5),
                        );
                    }
                }
            }
        }

        // Draw Connections
        for body in &universe.bodies {
            if let Some(parent_idx) = body.parent_id {
                let parent = &universe.bodies[parent_idx];
                draw_line(
                    body.pos.x, body.pos.y,
                    parent.pos.x, parent.pos.y,
                    0.5 / cam_zoom,
                    Color::new(0.2, 0.2, 0.8, 0.3),
                );
            }
        }

        // Draw Bodies
        for body in &universe.bodies {
            // Color based on mass?
            let color = if body.mass > 500.0 { YELLOW } else if body.mass > 50.0 { BLUE } else { WHITE };
            draw_circle(body.pos.x, body.pos.y, body.radius, color);

            // Draw Name
            if show_names {
                draw_text_ex(
                    &body.name,
                    body.pos.x + body.radius,
                    body.pos.y,
                    TextParams {
                        font_size: 30,
                        // Scale text so it's readable in world space.
                        // font_scale is multiplier.
                        font_scale: 0.1,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        // Draw HUD (Fixed Camera)
        set_default_camera();
        draw_text("ARROWS: Pan | SCROLL: Zoom | N: Names | T: Trails", 10.0, 20.0, 20.0, WHITE);
        draw_text(format!("Bodies: {}", universe.bodies.len()).as_str(), 10.0, 40.0, 20.0, WHITE);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 60.0, 20.0, WHITE);

        next_frame().await
    }
}
