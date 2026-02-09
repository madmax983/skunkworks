use macroquad::prelude::*;
use rayon::prelude::*;
use num_complex::Complex;

mod agent;
use agent::{Agent, SimParams, State};

#[macroquad::main("Hyperbolic Bridge")]
async fn main() {
    let width = 800;
    let height = 800;
    request_new_screen_size(width as f32, height as f32);

    let num_agents = 5000;

    // Initialize agents in the center
    let mut agents: Vec<Agent> = (0..num_agents).map(|_| {
        let r = rand::gen_range(0.0, 0.3); // Start safely inside
        let theta = rand::gen_range(0.0, std::f64::consts::PI * 2.0);
        let pos = Complex::from_polar(r, theta);
        let angle = rand::gen_range(0.0, std::f64::consts::PI * 2.0);
        Agent::new(pos, angle)
    }).collect();

    // Trail map (density field)
    let mut trail_map = vec![0.0f32; width * height];
    let mut next_map = vec![0.0f32; width * height];

    // Visualization texture
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    // Simulation params
    let params = SimParams {
        width,
        height,
        sensor_angle: 45.0f64.to_radians(),
        sensor_dist: 0.05,
        turn_speed: 0.2,
        move_speed: 0.01,
    };
    let deposit_amount = 0.5;
    let decay_factor = 0.95;

    loop {
        // 1. Update Agents
        let map_slice = &trail_map;
        agents.par_iter_mut().for_each(|agent| {
            agent.update(map_slice, params);
        });

        // 2. Deposit Pheromones (Sequential for now)
        for agent in &agents {
            if agent.pos.norm_sqr() < 1.0 {
                 let x = ((agent.pos.re + 1.0) * 0.5 * (width as f64)) as usize;
                 let y = ((agent.pos.im + 1.0) * 0.5 * (height as f64)) as usize;

                 if x < width && y < height {
                     trail_map[y * width + x] += deposit_amount;
                     if trail_map[y * width + x] > 10.0 {
                         trail_map[y * width + x] = 10.0;
                     }
                 }
            }
        }

        // 3. Diffuse & Decay (Parallel)
        next_map.par_iter_mut().enumerate().for_each(|(i, val)| {
            let x = i % width;
            let y = i / width;

            let mut sum = 0.0;
            let mut count = 0.0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        sum += trail_map[ny as usize * width + nx as usize];
                        count += 1.0;
                    }
                }
            }

            let blurred = sum / count;
            *val = blurred * decay_factor;
        });

        std::mem::swap(&mut trail_map, &mut next_map);

        // 4. Render to Texture (Pheromones)
        let bytes = &mut image.bytes;
        bytes.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
             if i < trail_map.len() {
                 let density = trail_map[i];
                 let val = (density * 255.0).min(255.0) as u8;

                 // Greenish pheromones
                 pixel[0] = 0;       // R
                 pixel[1] = val;     // G
                 pixel[2] = val / 2; // B
                 pixel[3] = 255;     // A
             }
        });

        texture.update(&image);

        // Draw
        clear_background(BLACK);

        let disk_center = vec2(width as f32 / 2.0, height as f32 / 2.0);
        let disk_radius = width as f32 / 2.0;

        // Draw Density Map
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Draw Gap Ring (Red boundaries)
        let r_inner = disk_radius * 0.4;
        let r_outer = disk_radius * 0.6;

        draw_circle_lines(disk_center.x, disk_center.y, r_inner, 2.0, RED);
        draw_circle_lines(disk_center.x, disk_center.y, r_outer, 2.0, RED);

        // Draw Boundary
        draw_circle_lines(disk_center.x, disk_center.y, disk_radius, 2.0, WHITE);

        // Draw Bridging Agents
        for agent in &agents {
            if agent.state == State::Bridging {
                let x = (agent.pos.re + 1.0) * 0.5 * (width as f64);
                let y = (agent.pos.im + 1.0) * 0.5 * (height as f64);
                draw_rectangle(x as f32 - 1.5, y as f32 - 1.5, 3.0, 3.0, YELLOW);
            }
        }

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", num_agents), 20.0, 50.0, 30.0, WHITE);
        let bridging_count = agents.iter().filter(|a| a.state == State::Bridging).count();
        draw_text(&format!("Bridges: {}", bridging_count), 20.0, 80.0, 30.0, YELLOW);

        next_frame().await
    }
}
