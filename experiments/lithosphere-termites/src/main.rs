use macroquad::prelude::*;
use macroquad::models::{Mesh, Vertex};
use crate::terrain::Terrain;
use crate::history::{HistoryStream, FileMapper, GeologicalEvent};
use crate::termites::TermiteColony;

mod terrain;
mod history;
mod termites;

#[macroquad::main("Lithosphere Termites")]
async fn main() {
    let width = 64; // Reduced for performance
    let height = 64;
    let mut terrain = Terrain::new(width, height);
    let mut mapper = FileMapper::new(width, height);
    let mut termites = TermiteColony::new(500, width as f32, height as f32);

    // Add some initial noise
    terrain.add_noise(2.0);

    // Try to load history, fallback to demo if fails
    let mut history = HistoryStream::new(".", 1000).ok();
    let mut demo_mode = false;
    if history.is_none() {
        println!("Could not load git history, running in demo mode");
        demo_mode = true;
    }

    // Camera
    let mut cam_pos = vec3(width as f32 * 0.5, 40.0, height as f32 * 1.2);
    let mut cam_target = vec3(width as f32 * 0.5, 0.0, height as f32 * 0.5);

    let mut paused = false;
    let mut speed = 1;
    let mut current_hash = String::from("START");
    let mut current_msg = String::from("Initializing...");

    loop {
        // Controls
        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_pressed(KeyCode::Equal) { speed += 1; }
        if is_key_pressed(KeyCode::Minus) { if speed > 1 { speed -= 1; } }

        let move_speed = 1.0;
        if is_key_down(KeyCode::Left) { cam_pos.x -= move_speed; cam_target.x -= move_speed; }
        if is_key_down(KeyCode::Right) { cam_pos.x += move_speed; cam_target.x += move_speed; }
        if is_key_down(KeyCode::Up) { cam_pos.z -= move_speed; cam_target.z -= move_speed; }
        if is_key_down(KeyCode::Down) { cam_pos.z += move_speed; cam_target.z += move_speed; }
        if is_key_down(KeyCode::PageUp) { cam_pos.y += move_speed; }
        if is_key_down(KeyCode::PageDown) { cam_pos.y -= move_speed; }

        if !paused {
            for _ in 0..speed {
                 // Process history event
                 if let Some(h) = &mut history {
                     if let Some((events, hash, msg)) = h.next_events(&mut mapper) {
                         current_hash = hash;
                         current_msg = msg;
                         for event in events {
                             match event {
                                 GeologicalEvent::Uplift { x, y, amount } => {
                                     terrain.uplift(x, y, amount);
                                 },
                                 GeologicalEvent::Weathering { x, y, intensity } => {
                                     // Weathering triggers erosion at that spot
                                     let iters = (intensity * 20.0) as usize;
                                     for _ in 0..iters {
                                         terrain.erode_at(x, y);
                                     }
                                 }
                             }
                         }
                     }
                 } else if demo_mode {
                     // Random uplift in demo mode
                     if rand::gen_range(0, 20) == 0 {
                         let x = rand::gen_range(0, width);
                         let y = rand::gen_range(0, height);
                         terrain.uplift(x, y, rand::gen_range(1.0, 5.0));
                         current_msg = format!("Random Uplift at {},{}", x, y);
                     }
                     // Random erosion
                     if rand::gen_range(0, 10) == 0 {
                         let x = rand::gen_range(0, width);
                         let y = rand::gen_range(0, height);
                         terrain.erode_at(x, y);
                     }
                 }

                 // Global erosion
                 terrain.erode(10);

                 // Termite Update
                 termites.update(&mut terrain);
            }
        }

        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position: cam_pos,
            target: cam_target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_terrain(&terrain);
        draw_termites(&termites, &terrain);

        set_default_camera();

        // UI
        draw_text(&format!("Commit: {} {}", current_hash, current_msg), 10.0, 30.0, 20.0, BLACK);
        draw_text(&format!("Speed: {}x | Paused: {}", speed, paused), 10.0, 50.0, 20.0, BLACK);
        draw_text(&format!("Termites: {}", termites.agents.len()), 10.0, 70.0, 20.0, BLACK);
        draw_text("Controls: Arrows/PgUp/PgDn to move, Space to pause, +/- speed", 10.0, 90.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn draw_terrain(terrain: &Terrain) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut idx = 0;

    for z in 0..terrain.height-1 {
        for x in 0..terrain.width-1 {
            let h00 = terrain.get_height(x, z);
            let h10 = terrain.get_height(x+1, z);
            let h01 = terrain.get_height(x, z+1);
            let h11 = terrain.get_height(x+1, z+1);

            let p00 = vec3(x as f32, h00, z as f32);
            let p10 = vec3((x+1) as f32, h10, z as f32);
            let p01 = vec3(x as f32, h01, (z+1) as f32);
            let p11 = vec3((x+1) as f32, h11, (z+1) as f32);

            // Access sediment directly if public? Yes, struct fields are public.
            let idx_00 = terrain.get_index(x, z);
            let sed = terrain.sediment[idx_00];
            let color = get_color(h00, sed);
            let color_bytes: [u8; 4] = color.into();

            // Triangle 1: p00, p01, p10 (CCW)
            let normal = vec4(0., 1., 0., 0.);

            vertices.push(Vertex { position: p00, uv: vec2(0.,0.), color: color_bytes, normal });
            vertices.push(Vertex { position: p01, uv: vec2(0.,0.), color: color_bytes, normal });
            vertices.push(Vertex { position: p10, uv: vec2(0.,0.), color: color_bytes, normal });
            indices.push(idx); indices.push(idx+1); indices.push(idx+2);
            idx += 3;

            // Triangle 2: p10, p01, p11 (CCW)
            vertices.push(Vertex { position: p10, uv: vec2(0.,0.), color: color_bytes, normal });
            vertices.push(Vertex { position: p01, uv: vec2(0.,0.), color: color_bytes, normal });
            vertices.push(Vertex { position: p11, uv: vec2(0.,0.), color: color_bytes, normal });
            indices.push(idx); indices.push(idx+1); indices.push(idx+2);
            idx += 3;
        }
    }

    let mesh = Mesh {
        vertices,
        indices,
        texture: None,
    };

    draw_mesh(&mesh);
}

fn draw_termites(colony: &TermiteColony, terrain: &Terrain) {
    for agent in &colony.agents {
        // Draw as small sphere/cube
        // Position at terrain height + small offset
        let h = terrain.get_height(agent.x as usize, agent.y as usize);
        let pos = vec3(agent.x, h + 0.5, agent.y);

        let color = if agent.carrying > 0.0 { RED } else { BLACK };

        // Sphere is expensive if many, use point/line?
        // Or manually add to mesh?
        // Let's use `draw_cube` for now, maybe only draw subset if too slow?
        // 500 agents is fine for macroquad.
        draw_cube(pos, vec3(0.3, 0.3, 0.3), None, color);
    }
}

fn get_color(height: f32, sediment: f32) -> Color {
    if sediment > 0.5 {
         return Color::new(0.8, 0.7, 0.4, 1.0); // Sand/Dirt
    }

    if height < 1.0 {
        Color::new(0.2, 0.8, 0.2, 1.0) // Grass
    } else if height < 8.0 {
        Color::new(0.5, 0.5, 0.5, 1.0) // Rock
    } else {
        WHITE // Snow
    }
}
