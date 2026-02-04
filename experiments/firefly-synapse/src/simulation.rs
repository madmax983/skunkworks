use rand::prelude::*;
use rayon::prelude::*;
use std::f32::consts::PI;

pub const AGENT_COUNT: usize = 100_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const INTERACTION_RADIUS: f32 = 4.0;
pub const COUPLING_STRENGTH: f32 = 1.2; // K
pub const DT: f32 = 0.1;
pub const NOISE: f32 = 0.05;

#[derive(Clone, Copy, Debug)]
pub struct Firefly {
    pub x: f32,
    pub y: f32,
    pub phase: f32, // 0 to 2PI
    pub natural_freq: f32,
}

pub struct World {
    pub agents: Vec<Firefly>,
    pub width: f32,
    pub height: f32,

    // Linked Cell List
    pub grid_head: Vec<Option<usize>>, // Indices into agents
    pub next_node: Vec<Option<usize>>, // Indices into agents

    pub grid_cols: usize,
    pub grid_rows: usize,
    pub cell_size: f32,
}

impl World {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        for _ in 0..AGENT_COUNT {
            agents.push(Firefly {
                x: rng.gen::<f32>() * WORLD_SIZE,
                y: rng.gen::<f32>() * WORLD_SIZE,
                phase: rng.gen::<f32>() * 2.0 * PI,
                natural_freq: 0.1 + rng.gen::<f32>() * 0.02,
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

        // Calculate phase updates
        let updates: Vec<(f32, f32, f32)> = agents
            .par_iter()
            .enumerate()
            .map(|(i, agent)| {
                let cx = (agent.x / cell_size).floor() as isize;
                let cy = (agent.y / cell_size).floor() as isize;

                let mut interaction_sum = 0.0;
                let mut count = 0;

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
                                    interaction_sum += (neighbor.phase - agent.phase).sin();
                                    count += 1;
                                }
                            }
                            current_node = next_node[neighbor_idx];
                        }
                    }
                }

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

                (new_phase, new_x, new_y)
            })
            .collect();

        // Apply updates
        self.agents
            .par_iter_mut()
            .zip(updates)
            .for_each(|(agent, (p, x, y))| {
                agent.phase = p;
                agent.x = x;
                agent.y = y;
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

        // Precompute color map? No, calculate on fly is cheap.
        for agent in &self.agents {
            let px = (agent.x * scale_x) as usize;
            let py = (agent.y * scale_y) as usize;

            if px < width && py < height {
                let idx = (py * width + px) * 4;

                // Intensity: Sharp peak when cos(phase) is near 1
                let cos_p = agent.phase.cos();
                let intensity = ((cos_p + 1.0) * 0.5).powf(20.0);

                let val = (intensity * 255.0) as u8;

                // "Max" blending to preserve brightest pulses
                if val > buffer[idx] {
                    buffer[idx] = val; // R
                    buffer[idx + 1] = val; // G (Yellow/White)
                    buffer[idx + 2] = val / 4; // B (Slightly warm)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synchronization_convergence() {
        let mut world = World::new();

        // Hijack
        world.agents[0].x = 500.0;
        world.agents[0].y = 500.0;
        world.agents[0].phase = 0.0;
        world.agents[0].natural_freq = 0.0;

        world.agents[1].x = 500.1;
        world.agents[1].y = 500.0;
        world.agents[1].phase = PI / 2.0;
        world.agents[1].natural_freq = 0.0;

        // Isolate others (remove them so they don't interfere and don't cause O(N^2) in one cell)
        world.agents.truncate(2);

        let diff_initial = (world.agents[0].phase - world.agents[1].phase).abs();

        world.update();

        let diff_after = (world.agents[0].phase - world.agents[1].phase).abs();

        assert!(
            diff_after < diff_initial,
            "Phases should converge. Initial: {}, After: {}",
            diff_initial,
            diff_after
        );
    }
}
