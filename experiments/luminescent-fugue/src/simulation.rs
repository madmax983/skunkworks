use rand::prelude::*;
use rayon::prelude::*;
use std::f32::consts::PI;

pub const AGENT_COUNT: usize = 100_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const INTERACTION_RADIUS: f32 = 5.0; // Slightly increased for better connectivity
pub const COUPLING_STRENGTH: f32 = 1.2;
pub const DT: f32 = 0.1;
pub const NOISE: f32 = 0.05;
pub const COLOR_MIXING_RATE: f32 = 0.1;

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub phase: f32, // 0 to 2PI
    pub natural_freq: f32,
    pub color: [f32; 3], // RGB [0.0, 1.0]
}

pub struct World {
    pub agents: Vec<Agent>,
    pub width: f32,
    pub height: f32,

    // Linked Cell List for spatial partitioning
    // grid_head[cell_index] -> index of first agent in cell
    pub grid_head: Vec<Option<usize>>,
    // next_node[agent_index] -> index of next agent in same cell
    pub next_node: Vec<Option<usize>>,

    pub grid_cols: usize,
    pub grid_rows: usize,
    pub cell_size: f32,
}

impl World {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        for _ in 0..AGENT_COUNT {
            agents.push(Agent {
                x: rng.gen::<f32>() * WORLD_SIZE,
                y: rng.gen::<f32>() * WORLD_SIZE,
                phase: rng.gen::<f32>() * 2.0 * PI,
                natural_freq: 0.1 + rng.gen::<f32>() * 0.02,
                color: [rng.gen(), rng.gen(), rng.gen()],
            });
        }

        let cell_size = INTERACTION_RADIUS;
        let grid_cols = (WORLD_SIZE / cell_size).ceil() as usize;
        let grid_rows = (WORLD_SIZE / cell_size).ceil() as usize;

        // Initialize linked list arrays
        let grid_head = vec![None; grid_cols * grid_rows];
        let next_node = vec![None; AGENT_COUNT];

        World {
            agents,
            width: WORLD_SIZE,
            height: WORLD_SIZE,
            grid_head,
            next_node,
            grid_cols,
            grid_rows,
            cell_size,
        }
    }

    pub fn update(&mut self) {
        self.update_grid();
        self.synchronize_and_move();
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        for agent in &mut self.agents {
             agent.x = rng.gen::<f32>() * WORLD_SIZE;
             agent.y = rng.gen::<f32>() * WORLD_SIZE;
             agent.phase = rng.gen::<f32>() * 2.0 * PI;
             agent.color = [rng.gen(), rng.gen(), rng.gen()];
        }
    }

    pub fn inject_chaos(&mut self) {
        let mut rng = rand::thread_rng();
        for agent in &mut self.agents {
             agent.phase = rng.gen::<f32>() * 2.0 * PI;
             // Don't reset color, let's see if they recover or drift
        }
    }

    // Inject a "Source" signal (force some agents to a specific color/phase)
    // useful for testing propagation
    pub fn inject_signal(&mut self, x: f32, y: f32, radius: f32, color: [f32; 3]) {
        for agent in &mut self.agents {
            let dx = agent.x - x;
            let dy = agent.y - y;
            if dx*dx + dy*dy < radius*radius {
                agent.color = color;
                agent.phase = 0.0; // Force sync
            }
        }
    }

    fn update_grid(&mut self) {
        // Reset grid heads
        self.grid_head.fill(None);

        // Build linked list
        // Note: This must be serial because we mutate grid_head randomly.
        // It's fast enough for 100k agents (approx 2-3ms).
        for (i, agent) in self.agents.iter().enumerate() {
            let cx = (agent.x / self.cell_size).floor() as usize;
            let cy = (agent.y / self.cell_size).floor() as usize;

            // Boundary checks (clamp to be safe, though wrap logic handles it)
            let cx = cx.min(self.grid_cols - 1);
            let cy = cy.min(self.grid_rows - 1);

            let cell_idx = cy * self.grid_cols + cx;

            self.next_node[i] = self.grid_head[cell_idx];
            self.grid_head[cell_idx] = Some(i);
        }
    }

    fn synchronize_and_move(&mut self) {
        // Compute new phases and positions in parallel
        // We need read-only access to agents and grid structure
        let agents = &self.agents;
        let grid_head = &self.grid_head;
        let next_node = &self.next_node;
        let grid_cols = self.grid_cols;
        let grid_rows = self.grid_rows;
        let cell_size = self.cell_size;
        let width = self.width;
        let height = self.height;

        // Calculate updates
        // Return type: (new_phase, new_x, new_y, new_color)
        let updates: Vec<(f32, f32, f32, [f32; 3])> = agents
            .par_iter()
            .enumerate()
            .map(|(i, agent)| {
                let cx = (agent.x / cell_size).floor() as isize;
                let cy = (agent.y / cell_size).floor() as isize;

                let mut interaction_sum = 0.0;
                let mut count = 0;

                let mut color_accum = [0.0, 0.0, 0.0];
                let mut color_weight_sum = 0.0;

                // Check 3x3 neighbors
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = cx + dx;
                        let ny = cy + dy;

                        // Wrap grid coordinates
                        let wrapped_nx =
                            (nx + grid_cols as isize).rem_euclid(grid_cols as isize) as usize;
                        let wrapped_ny =
                            (ny + grid_rows as isize).rem_euclid(grid_rows as isize) as usize;

                        let cell_idx = wrapped_ny * grid_cols + wrapped_nx;

                        // Iterate linked list for this cell
                        let mut current_node = grid_head[cell_idx];
                        while let Some(neighbor_idx) = current_node {
                            if neighbor_idx != i {
                                let neighbor = &agents[neighbor_idx];
                                // Distance check (squared)
                                let dx_pos = (neighbor.x - agent.x).abs();
                                let dy_pos = (neighbor.y - agent.y).abs();
                                // Handle toroidal wrapping for distance
                                let dx_pos = if dx_pos > width / 2.0 {
                                    width - dx_pos
                                } else {
                                    dx_pos
                                };
                                let dy_pos = if dy_pos > height / 2.0 {
                                    height - dy_pos
                                } else {
                                    dy_pos
                                };

                                if dx_pos * dx_pos + dy_pos * dy_pos
                                    < INTERACTION_RADIUS * INTERACTION_RADIUS
                                {
                                    // Phase coupling (Kuramoto)
                                    interaction_sum += (neighbor.phase - agent.phase).sin();
                                    count += 1;

                                    // Color consensus
                                    // Only influence color if neighbor is "bright" (phase near peak)
                                    // cos(phase) peaks at 0/2PI.
                                    // Let's use (cos(phase) + 1.0) / 2.0 as "visibility"
                                    let visibility = (neighbor.phase.cos() + 1.0) * 0.5;
                                    // Sharpen it so only very bright ones count
                                    let visibility = visibility.powf(4.0);

                                    if visibility > 0.01 {
                                        color_accum[0] += neighbor.color[0] * visibility;
                                        color_accum[1] += neighbor.color[1] * visibility;
                                        color_accum[2] += neighbor.color[2] * visibility;
                                        color_weight_sum += visibility;
                                    }
                                }
                            }
                            current_node = next_node[neighbor_idx];
                        }
                    }
                }

                // Phase Update
                let coupling = if count > 0 {
                    (COUPLING_STRENGTH / (count as f32)) * interaction_sum
                } else {
                    0.0
                };
                let d_theta = agent.natural_freq + coupling;
                let mut new_phase = agent.phase + d_theta * DT;
                // Wrap phase
                if new_phase > 2.0 * PI {
                    new_phase -= 2.0 * PI;
                } else if new_phase < 0.0 {
                     new_phase += 2.0 * PI;
                }

                // Color Update
                let mut new_color = agent.color;
                if color_weight_sum > 0.0 {
                    let target_r = color_accum[0] / color_weight_sum;
                    let target_g = color_accum[1] / color_weight_sum;
                    let target_b = color_accum[2] / color_weight_sum;

                    // Lerp towards target
                    new_color[0] += (target_r - new_color[0]) * COLOR_MIXING_RATE * DT;
                    new_color[1] += (target_g - new_color[1]) * COLOR_MIXING_RATE * DT;
                    new_color[2] += (target_b - new_color[2]) * COLOR_MIXING_RATE * DT;
                }

                // Move randomly
                let mut rng = rand::thread_rng();
                let move_x = (rng.gen::<f32>() - 0.5) * NOISE;
                let move_y = (rng.gen::<f32>() - 0.5) * NOISE;

                let mut new_x = agent.x + move_x;
                let mut new_y = agent.y + move_y;

                // Wrap position
                if new_x < 0.0 {
                    new_x += width;
                }
                if new_x >= width {
                    new_x -= width;
                }
                if new_y < 0.0 {
                    new_y += height;
                }
                if new_y >= height {
                    new_y -= height;
                }

                (new_phase, new_x, new_y, new_color)
            })
            .collect();

        // Apply updates
        self.agents
            .par_iter_mut()
            .zip(updates)
            .for_each(|(agent, (p, x, y, c))| {
                agent.phase = p;
                agent.x = x;
                agent.y = y;
                agent.color = c;
            });
    }

    pub fn render_to_buffer(&self, buffer: &mut [u8], width: usize, height: usize) {
        // Clear buffer
        buffer.par_chunks_exact_mut(4).for_each(|pixel| {
            pixel[0] = 0; // R
            pixel[1] = 0; // G
            pixel[2] = 0; // B
            pixel[3] = 255; // A
        });

        // Scatter write (Serial)
        let scale_x = width as f32 / self.width;
        let scale_y = height as f32 / self.height;

        for agent in &self.agents {
            let px = (agent.x * scale_x) as usize;
            let py = (agent.y * scale_y) as usize;

            if px < width && py < height {
                let idx = (py * width + px) * 4;

                // Brightness based on phase
                // Pulse when phase is near 0 or 2PI
                let cos_p = agent.phase.cos();
                let brightness = ((cos_p + 1.0) * 0.5).powf(10.0); // Sharp peak

                // Color from agent state
                let r = (agent.color[0] * brightness * 255.0) as u8;
                let g = (agent.color[1] * brightness * 255.0) as u8;
                let b = (agent.color[2] * brightness * 255.0) as u8;

                // Additive blending or Max blending?
                // Max blending is better for points to avoid saturation whiteout
                // But Additive looks more "glowy".
                // Let's use Max for now to keep colors distinct.

                if r > buffer[idx] { buffer[idx] = r; }
                if g > buffer[idx+1] { buffer[idx+1] = g; }
                if b > buffer[idx+2] { buffer[idx+2] = b; }
            }
        }
    }

    // Calculate consensus metric (std dev of colors)
    pub fn calculate_consensus_metric(&self) -> f32 {
        let n = self.agents.len() as f32;
        let sum_r: f32 = self.agents.iter().map(|a| a.color[0]).sum();
        let sum_g: f32 = self.agents.iter().map(|a| a.color[1]).sum();
        let sum_b: f32 = self.agents.iter().map(|a| a.color[2]).sum();

        let mean_r = sum_r / n;
        let mean_g = sum_g / n;
        let mean_b = sum_b / n;

        let variance_r: f32 = self.agents.iter().map(|a| (a.color[0] - mean_r).powi(2)).sum();
        let variance_g: f32 = self.agents.iter().map(|a| (a.color[1] - mean_g).powi(2)).sum();
        let variance_b: f32 = self.agents.iter().map(|a| (a.color[2] - mean_b).powi(2)).sum();

        ((variance_r + variance_g + variance_b) / n).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_convergence() {
        let mut world = World::new();
        // Setup 2 agents close to each other
        world.agents[0].x = 500.0;
        world.agents[0].y = 500.0;
        world.agents[0].phase = 0.0;
        world.agents[0].natural_freq = 0.0;

        world.agents[1].x = 500.1;
        world.agents[1].y = 500.0;
        world.agents[1].phase = PI / 2.0; // 90 degrees apart
        world.agents[1].natural_freq = 0.0;

        // Remove others
        world.agents.truncate(2);

        let initial_diff = (world.agents[0].phase - world.agents[1].phase).abs();

        // Run for a few steps
        for _ in 0..10 {
            world.update();
        }

        let final_diff = (world.agents[0].phase - world.agents[1].phase).abs();

        assert!(final_diff < initial_diff, "Phases should converge. Init: {}, Final: {}", initial_diff, final_diff);
    }

    #[test]
    fn test_color_consensus() {
        let mut world = World::new();
        // Setup 2 agents close to each other
        world.agents[0].x = 500.0;
        world.agents[0].y = 500.0;
        world.agents[0].phase = 0.0; // Both at peak brightness so they see each other
        world.agents[0].color = [1.0, 0.0, 0.0]; // Red

        world.agents[1].x = 500.1;
        world.agents[1].y = 500.0;
        world.agents[1].phase = 0.0; // Both at peak brightness
        world.agents[1].color = [0.0, 0.0, 1.0]; // Blue

        // Remove others
        world.agents.truncate(2);

        let initial_diff = (world.agents[0].color[0] - world.agents[1].color[0]).abs();

        // Run for a few steps
        for _ in 0..10 {
            world.update();
        }

        let final_diff = (world.agents[0].color[0] - world.agents[1].color[0]).abs();

        assert!(final_diff < initial_diff, "Colors should converge. Init: {}, Final: {}", initial_diff, final_diff);
    }
}
