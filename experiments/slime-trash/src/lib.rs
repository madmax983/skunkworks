use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgentState {
    SeekingTrash, // Empty, looking for trash. Follows trash_trails. Deposits dump_trails.
    SeekingDump,  // Full, looking for dump. Follows dump_trails. Deposits trash_trails.
}

#[derive(Clone)]
pub struct Agent {
    pub position: (f32, f32),
    pub angle: f32,
    pub cargo: f32,
    pub state: AgentState,
}

impl Agent {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: (x, y),
            angle: rng.gen_range(0.0..std::f32::consts::TAU),
            cargo: 0.0,
            state: AgentState::SeekingTrash,
        }
    }
}

pub struct World {
    pub agents: Vec<Agent>,
    pub trash_piles: Vec<(f32, f32, f32)>, // x, y, amount
    pub dumps: Vec<(f32, f32)>,            // x, y
    pub trash_trails: Vec<f32>,            // Pheromone grid A (Attracts seeking trash)
    pub dump_trails: Vec<f32>,             // Pheromone grid B (Attracts seeking dump)
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            agents: Vec::new(),
            trash_piles: Vec::new(),
            dumps: Vec::new(),
            trash_trails: vec![0.0; width * height],
            dump_trails: vec![0.0; width * height],
            width,
            height,
        }
    }

    pub fn add_agent(&mut self, x: f32, y: f32) {
        self.agents.push(Agent::new(x, y));
    }

    pub fn add_trash(&mut self, x: f32, y: f32, amount: f32) {
        self.trash_piles.push((x, y, amount));
    }

    pub fn add_dump(&mut self, x: f32, y: f32) {
        self.dumps.push((x, y));
    }

    pub fn update(&mut self, dt: f32) {
        let pickup_radius = 5.0;
        let drop_radius = 5.0;
        let pickup_rate = 100.0; // Amount per second

        // 1. Move Agents & Deposit Pheromones
        for i in 0..self.agents.len() {
            // Calculate turning based on sensors
            let (turn_angle, _speed_mod) = {
                let agent = &self.agents[i];
                let grid = match agent.state {
                    AgentState::SeekingTrash => &self.trash_trails,
                    AgentState::SeekingDump => &self.dump_trails,
                };
                Self::calculate_turn(agent, grid, self.width, self.height)
            };

            // Apply turn
            self.agents[i].angle += turn_angle;

            // Move
            let speed = if self.agents[i].cargo > 0.0 {
                15.0
            } else {
                40.0
            };
            let dx = self.agents[i].angle.cos() * speed * dt;
            let dy = self.agents[i].angle.sin() * speed * dt;

            let mut next_x = self.agents[i].position.0 + dx;
            let mut next_y = self.agents[i].position.1 + dy;

            // Bounce
            if next_x < 0.0 || next_x >= self.width as f32 {
                self.agents[i].angle = std::f32::consts::PI - self.agents[i].angle;
                next_x = next_x.clamp(0.0, self.width as f32 - 0.1);
            }
            if next_y < 0.0 || next_y >= self.height as f32 {
                self.agents[i].angle = -self.agents[i].angle;
                next_y = next_y.clamp(0.0, self.height as f32 - 0.1);
            }
            self.agents[i].position = (next_x, next_y);

            // Deposit Pheromones
            let ix = (next_x as usize).clamp(0, self.width - 1);
            let iy = (next_y as usize).clamp(0, self.height - 1);
            let idx = iy * self.width + ix;

            let deposit_amount = 0.5;
            match self.agents[i].state {
                AgentState::SeekingTrash => {
                    self.dump_trails[idx] = (self.dump_trails[idx] + deposit_amount).min(10.0);
                }
                AgentState::SeekingDump => {
                    self.trash_trails[idx] = (self.trash_trails[idx] + deposit_amount).min(10.0);
                }
            }
        }

        // 2. Logic Interactions (Pickup/Drop)
        for agent in &mut self.agents {
            match agent.state {
                AgentState::SeekingTrash => {
                    for trash in &mut self.trash_piles {
                        let dx = agent.position.0 - trash.0;
                        let dy = agent.position.1 - trash.1;
                        let dist_sq = dx * dx + dy * dy;

                        if dist_sq < pickup_radius * pickup_radius && trash.2 > 0.0 {
                            let grab = (pickup_rate * dt).min(trash.2).min(10.0 - agent.cargo);
                            if grab > 0.0 {
                                trash.2 -= grab;
                                agent.cargo += grab;
                                // Immediate switch logic (Physarum style)
                                agent.state = AgentState::SeekingDump;
                                agent.angle += std::f32::consts::PI;
                            }
                        }
                    }
                }
                AgentState::SeekingDump => {
                    for dump in &self.dumps {
                        let dx = agent.position.0 - dump.0;
                        let dy = agent.position.1 - dump.1;
                        let dist_sq = dx * dx + dy * dy;

                        if dist_sq < drop_radius * drop_radius {
                            // Drop all
                            agent.cargo = 0.0;
                            agent.state = AgentState::SeekingTrash;
                            agent.angle += std::f32::consts::PI;
                            break;
                        }
                    }
                }
            }
        }

        // 3. Diffuse & Decay
        let decay = 0.95;
        for v in &mut self.trash_trails {
            *v *= decay;
        }
        for v in &mut self.dump_trails {
            *v *= decay;
        }

        // Remove empty trash
        self.trash_piles.retain(|t| t.2 > 0.001);
    }

    fn calculate_turn(agent: &Agent, grid: &[f32], width: usize, height: usize) -> (f32, f32) {
        let sensor_angle = std::f32::consts::PI / 4.0;
        let sensor_dist = 15.0;

        let get_phero = |angle_offset: f32| -> f32 {
            let angle = agent.angle + angle_offset;
            let sx = agent.position.0 + angle.cos() * sensor_dist;
            let sy = agent.position.1 + angle.sin() * sensor_dist;

            if sx < 0.0 || sx >= width as f32 || sy < 0.0 || sy >= height as f32 {
                return 0.0;
            }

            let ix = sx as usize;
            let iy = sy as usize;
            grid[iy * width + ix]
        };

        let left = get_phero(-sensor_angle);
        let center = get_phero(0.0);
        let right = get_phero(sensor_angle);

        let mut rng = rand::thread_rng();

        if center > left && center > right {
            (0.0, 1.0) // Keep going
        } else if center < left && center < right {
            // Confused / surrounded? Random turn
            ((rng.gen::<f32>() - 0.5) * 2.0, 0.5)
        } else if left < right {
            // Right is stronger. Turn Right (CW)
            (sensor_angle * 0.2, 1.0)
        } else if right < left {
            // Left is stronger. Turn Left (CCW)
            (-sensor_angle * 0.2, 1.0)
        } else {
            // Wander
            ((rng.gen::<f32>() - 0.5) * 0.2, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_picks_up_trash() {
        let mut world = World::new(100, 100);
        world.add_trash(10.0, 10.0, 100.0);
        let mut agent = Agent::new(10.0, 10.0);
        agent.state = AgentState::SeekingTrash;
        world.agents.push(agent);

        world.update(0.1);

        let agent = &world.agents[0];
        assert_eq!(agent.state, AgentState::SeekingDump);
        assert!(agent.cargo > 0.0);
        assert!(world.trash_piles[0].2 < 100.0);
    }

    #[test]
    fn test_agent_drops_trash() {
        let mut world = World::new(100, 100);
        world.add_dump(90.0, 90.0);
        let mut agent = Agent::new(90.0, 90.0);
        agent.state = AgentState::SeekingDump;
        agent.cargo = 10.0;
        world.agents.push(agent);

        world.update(0.1);

        let agent = &world.agents[0];
        assert_eq!(agent.state, AgentState::SeekingTrash);
        assert_eq!(agent.cargo, 0.0);
    }
}
