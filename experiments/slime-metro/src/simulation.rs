use macroquad::prelude::*;
use rayon::prelude::*;
use crate::agent::{Agent, SimParams};
use crate::map::Map;

pub struct Simulation {
    pub map: Map,
    pub agents: Vec<Agent>,
    pub params: SimParams,
    pub frame_count: u64,
}

impl Simulation {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let mut map = Map::new(width, height);

        // Generate Cities (Food Sources)
        let num_cities = 12;
        let mut cities = Vec::new();
        for _ in 0..num_cities {
            let cx = fastrand::usize(width / 10..width - width / 10);
            let cy = fastrand::usize(height / 10..height - height / 10);
            cities.push((cx, cy));

            // Draw city on food map (gaussian splat)
            for dy in -20..=20 {
                for dx in -20..=20 {
                    let x = cx as isize + dx;
                    let y = cy as isize + dy;
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        let dist = (dx*dx + dy*dy) as f32;
                        let val = (-dist / 100.0).exp();
                        map.food[y as usize * width + x as usize] += val;
                    }
                }
            }
        }

        // Spawn Agents at Cities
        let mut agents = Vec::with_capacity(num_agents);
        for _ in 0..num_agents {
            let city_idx = fastrand::usize(0..cities.len());
            let (cx, cy) = cities[city_idx];

            // Random offset
            let r = fastrand::f32() * 10.0;
            let theta = fastrand::f32() * std::f32::consts::PI * 2.0;
            let x = cx as f32 + r * theta.cos();
            let y = cy as f32 + r * theta.sin();

            let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
            agents.push(Agent::new(vec2(x, y), angle));
        }

        let params = SimParams {
            width,
            height,
            sensor_angle: 45.0f32.to_radians(),
            sensor_dist: 9.0,
            turn_speed: 0.2, // Radians per tick
            move_speed: 1.0, // Pixels per tick
        };

        Self {
            map,
            agents,
            params,
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        // 1. Update Agents (Parallel)
        // Access trail map (read-only)
        let trail_map = &self.map.trail;
        let params = self.params;

        self.agents.par_iter_mut().for_each(|agent| {
            agent.update(trail_map, params);
        });

        // 2. Deposit Trails (Sequential for now to avoid atomic complexity)
        // Agents deposit based on their cargo level? For now just constant.
        let width = self.map.width;
        let height = self.map.height;
        let trail_map = &mut self.map.trail;
        let food_map = &self.map.food; // Read food map for interactions

        // We iterate agents to deposit
        // Also check if agent is at food source to pick up cargo/deposit cargo logic?
        // Let's keep it simple: Standard Physarum deposit first.

        let deposit_amount = 5.0; // High deposit to counter strong decay

        for agent in &mut self.agents {
            let x = agent.pos.x as usize;
            let y = agent.pos.y as usize;

            if x < width && y < height {
                let idx = y * width + x;
                trail_map[idx] = (trail_map[idx] + deposit_amount).min(100.0); // Cap it

                // Interaction with food
                let food_val = food_map[idx];
                if food_val > 0.5 {
                    // At a station
                    // If cargo is low, pick up
                    if agent.cargo < 0.5 {
                        agent.cargo += 0.1;
                    } else {
                        // If cargo is high, drop off (maybe at a different station?)
                        // For now just oscillate cargo to visualize flow
                        agent.cargo -= 0.1;
                    }
                }
            }
        }

        // 3. Diffuse and Decay Map
        // Decay rate
        let decay_rate = 0.90;
        self.map.diffuse_and_decay(decay_rate);

        // 4. Update Texture for rendering
        self.map.update_texture();
    }
}
