use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Agent {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub is_bid: bool,
    pub active: bool,
}

pub struct AgentUpdateResult {
    pub deposit_x: usize,
    pub deposit_y: usize,
    pub deposit_amount: f64,
}

impl Agent {
    pub fn new(x: f64, y: f64, angle: f64, is_bid: bool, _id: usize) -> Self {
        Self {
            x,
            y,
            angle,
            is_bid,
            active: true,
        }
    }

    /// Senses pheromone trail. Bids and Asks use the same trail but are attracted to it
    pub fn sense(&self, world: &World, angle_offset: f64, sensor_dist: f64) -> f64 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.y + sensor_angle.sin() * sensor_dist;

        let w = world.width as f64;
        let h = world.height as f64;
        let sx = (sensor_x.rem_euclid(w)) as usize;
        let sy = (sensor_y.rem_euclid(h)) as usize;

        // Bids want to go UP overall (y-). Asks want to go DOWN overall (y+).
        // Let's add a gradient attraction towards the opposite side of the market.
        let target_y = if self.is_bid { 0.0 } else { h - 1.0 };

        let dy = target_y - sensor_y;
        let dx = 0.0; // Straight up/down bias
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt().max(1.0);
        let gradient_strength = 500.0 / dist; // Similar to myco-transit logic

        let trail_strength = world.get_trail(sx, sy);

        trail_strength + gradient_strength
    }

    pub fn update(&mut self, world: &World) -> AgentUpdateResult {
        if !self.active {
            return AgentUpdateResult {
                deposit_x: 0,
                deposit_y: 0,
                deposit_amount: 0.0,
            };
        }

        // Bids naturally face upwards (-PI/2), Asks face downwards (PI/2)
        // We let them steer
        let sensor_angle = PI / 4.0;
        let sensor_dist = 4.0;
        let turn_angle = PI / 8.0;
        let speed = 1.0;

        let left = self.sense(world, -sensor_angle, sensor_dist);
        let center = self.sense(world, 0.0, sensor_dist);
        let right = self.sense(world, sensor_angle, sensor_dist);

        let mut rng = rand::thread_rng();

        if center > left && center > right {
            // Keep going
        } else if center < left && center < right {
            // Random turn
            if rng.gen_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
        } else if left > right {
            self.angle -= turn_angle;
        } else if right > left {
            self.angle += turn_angle;
        }

        // Move
        self.x += self.angle.cos() * speed;
        self.y += self.angle.sin() * speed;

        // Clamp to market bounds instead of wrapping like planets
        self.x = self.x.clamp(0.0, world.width as f64 - 1.1);
        self.y = self.y.clamp(0.0, world.height as f64 - 1.1);

        let ix = self.x as usize;
        let iy = self.y as usize;

        // Deposit a small trail just by moving
        AgentUpdateResult {
            deposit_x: ix,
            deposit_y: iy,
            deposit_amount: 1.0,
        }
    }
}

pub struct TradeEvent {
    pub x: usize,
    pub y: usize,
    pub age: u8,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub trails: Vec<f64>,
    pub next_trails: Vec<f64>,
    pub agents: Vec<Agent>,
    pub trades: Vec<TradeEvent>,
    pub next_agent_id: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            trails: vec![0.0; width * height],
            next_trails: vec![0.0; width * height],
            agents: Vec::new(),
            trades: Vec::new(),
            next_agent_id: 1,
        }
    }

    pub fn get_trail(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.trails[y * self.width + x]
    }

    pub fn spawn_bid(&mut self, x: usize, y: usize) {
        self.agents.push(Agent::new(
            x as f64,
            y as f64,
            -PI / 2.0,
            true,
            self.next_agent_id,
        ));
        self.next_agent_id += 1;
    }

    pub fn spawn_ask(&mut self, x: usize, y: usize) {
        self.agents.push(Agent::new(
            x as f64,
            y as f64,
            PI / 2.0,
            false,
            self.next_agent_id,
        ));
        self.next_agent_id += 1;
    }

    pub fn update(&mut self) {
        // Step 1: Update agents and get deposits
        let mut agents_clone = self.agents.clone();
        let mut deposits = Vec::new();
        for agent in agents_clone.iter_mut() {
            deposits.push(agent.update(self));
        }
        self.agents = agents_clone;

        for deposit in deposits.iter() {
            if deposit.deposit_amount > 0.0
                && deposit.deposit_x < self.width
                && deposit.deposit_y < self.height
            {
                let idx = deposit.deposit_y * self.width + deposit.deposit_x;
                self.trails[idx] = (self.trails[idx] + deposit.deposit_amount).min(255.0);
            }
        }

        // Step 2: Check for collisions (Trades)
        let mut new_trades = Vec::new();
        // Simple O(N^2) collision for agents that are close.
        // We use active flag to avoid double trading.
        let mut to_deactivate = vec![false; self.agents.len()];
        for i in 0..self.agents.len() {
            if !self.agents[i].active || to_deactivate[i] {
                continue;
            }
            for j in (i + 1)..self.agents.len() {
                if !self.agents[j].active || to_deactivate[j] {
                    continue;
                }

                if self.agents[i].is_bid != self.agents[j].is_bid {
                    let dx = self.agents[i].x - self.agents[j].x;
                    let dy = self.agents[i].y - self.agents[j].y;
                    if dx * dx + dy * dy < 2.0 {
                        // Trade!
                        to_deactivate[i] = true;
                        to_deactivate[j] = true;

                        let tx = ((self.agents[i].x + self.agents[j].x) / 2.0) as usize;
                        let ty = ((self.agents[i].y + self.agents[j].y) / 2.0) as usize;

                        new_trades.push(TradeEvent {
                            x: tx,
                            y: ty,
                            age: 15, // Display for 15 frames
                        });

                        // Massive pheromone deposit on trade
                        if tx < self.width && ty < self.height {
                            let idx = ty * self.width + tx;
                            self.trails[idx] = 255.0; // Huge burst of liquidity pheromones
                        }
                    }
                }
            }
        }

        for (i, agent) in self.agents.iter_mut().enumerate() {
            if to_deactivate[i] {
                agent.active = false;
            }
        }

        // Remove inactive agents
        self.agents.retain(|a| a.active);

        // Remove agents that reached the opposite end without trading (expired orders)
        self.agents.retain(|a| {
            if a.is_bid && a.y < 2.0 {
                return false;
            } // Reached top
            if !a.is_bid && a.y > (self.height as f64 - 3.0) {
                return false;
            } // Reached bottom
            true
        });

        // Decay trades
        for trade in self.trades.iter_mut() {
            if trade.age > 0 {
                trade.age -= 1;
            }
        }
        self.trades.retain(|t| t.age > 0);
        self.trades.extend(new_trades);

        // Diffuse and Decay trails
        let decay_factor = 0.95;
        let width = self.width;
        let height = self.height;
        let trails_ref = &self.trails;

        self.next_trails
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).clamp(0, width as isize - 1) as usize;
                            let ny = (y as isize + dy).clamp(0, height as isize - 1) as usize;
                            sum += trails_ref[ny * width + nx];
                        }
                    }
                    let avg = sum / 9.0;
                    *pixel = avg * decay_factor;
                }
            });

        std::mem::swap(&mut self.trails, &mut self.next_trails);
    }
}
