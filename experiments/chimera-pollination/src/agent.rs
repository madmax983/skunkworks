use crate::math::Vec4D;
use crate::monitor::SystemMonitor;
use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Dna {
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub pollination_weight: f32,
    pub view_radius: f32,
    pub max_speed: f32,
    pub max_force: f32,
}

#[derive(Clone)]
pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Vec4D,
    pub vel: Vec4D,
    pub acc: Vec4D,
    pub dna_params: Dna,
    pub color: Color,
    pub age: u32,
    pub id: u64,
}

impl Agent {
    pub fn new_random(id: u64) -> Self {
        let mut rng = ::rand::thread_rng();

        // 1. Generate VM DNA (Brain)
        let mut genes = Vec::new();
        for _ in 0..32 {
            let op = match rng.gen_range(0..12) {
                0 => OpCode::Push,
                1 => OpCode::Drop,
                2 => OpCode::Add,
                3 => OpCode::Sub,
                4 => OpCode::Mul,
                5 => OpCode::Div,
                6 => OpCode::Migrate,
                7 => OpCode::Signal,
                8 => OpCode::Receive,
                9 => OpCode::Jump,
                10 => OpCode::Brz,
                11 => OpCode::Dup,
                _ => OpCode::Nop,
            };

            let args = if op == OpCode::Push {
                vec![Nucleotide::Number(rng.gen_range(0..100))]
            } else if op == OpCode::Jump || op == OpCode::Brz {
                vec![Nucleotide::Number(rng.gen_range(0..32))]
            } else {
                vec![]
            };

            genes.push(Gene { op, args });
        }

        let dna = chimera_lang::prelude::Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        // 2. Generate Physical DNA (Body)
        let dna_params = Dna {
            separation_weight: rng.gen_range(1.0..2.0),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            pollination_weight: rng.gen_range(0.5..1.5),
            view_radius: rng.gen_range(1.5..3.0),
            max_speed: rng.gen_range(0.05..0.1),
            max_force: rng.gen_range(0.005..0.02),
        };

        // 3. Initial State
        let pos = Vec4D::new(
            rng.gen_range(-2.0..2.0),
            rng.gen_range(-2.0..2.0),
            rng.gen_range(-2.0..2.0),
            rng.gen_range(-2.0..2.0),
        );
        let vel = Vec4D::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
        .scale(dna_params.max_speed);

        Self {
            vm,
            pos,
            vel,
            acc: Vec4D::zero(),
            dna_params,
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
            age: 0,
            id,
        }
    }

    pub fn update(
        &mut self,
        agents: &[Agent],
        monitor: &SystemMonitor,
        nearest_plant_vec: Option<Vec4D>,
        bounds: Vec4D,
    ) {
        self.age += 1;
        self.vm.energy = 1000;

        // --- Step 1: Calculate Flocking Inputs ---
        let separation = self
            .separation(agents)
            .scale(self.dna_params.separation_weight);
        let alignment = self
            .alignment(agents)
            .scale(self.dna_params.alignment_weight);
        let cohesion = self.cohesion(agents).scale(self.dna_params.cohesion_weight);

        let flocking_force = separation + alignment + cohesion;
        let plant_force = if let Some(dir) = nearest_plant_vec {
            dir.normalize().scale(self.dna_params.pollination_weight)
        } else {
            Vec4D::zero()
        };

        // --- Step 2: Feed Inputs to VM (Ether) ---
        let fx = (flocking_force.x * 100.0) as i64;
        let fy = (flocking_force.y * 100.0) as i64;
        let fz = (flocking_force.z * 100.0) as i64;
        let fw = (flocking_force.w * 100.0) as i64;

        let px = (plant_force.x * 100.0) as i64;
        let py = (plant_force.y * 100.0) as i64;

        let wind = (monitor.cpu_usage * 100.0) as i64;

        self.push_input(0, fx);
        self.push_input(0, fy);
        self.push_input(1, fz);
        self.push_input(1, fw);
        self.push_input(2, px);
        self.push_input(2, py);
        self.push_input(3, wind);

        // --- Step 3: Run VM ---
        for _ in 0..10 {
            self.vm.step();
        }

        // --- Step 4: Read Outputs ---
        // Reading "Volition" from VM
        let vol_x = self.sum_channel(4) * 0.001;
        let vol_y = self.sum_channel(5) * 0.001;
        let vol_z = self.sum_channel(6) * 0.001;
        let vol_w = self.sum_channel(7) * 0.001;

        let volition = Vec4D::new(vol_x, vol_y, vol_z, vol_w);

        // --- Step 5: Apply Physics ---
        self.acc += flocking_force;
        self.acc += plant_force;
        self.acc += volition;

        // Add System Wind (Turbulence)
        let wind_vec = Vec4D::new(
            monitor.cpu_usage * 0.001,
            monitor.mem_usage * 0.001,
            monitor.swap_usage * 0.001,
            0.0,
        );
        self.acc += wind_vec;

        // Update velocity and position
        self.vel += self.acc;
        self.vel = self.vel.limit(self.dna_params.max_speed);
        self.pos += self.vel;
        self.acc = Vec4D::zero();

        // Bounds Checking (Bounce)
        self.bounce(bounds);
    }

    fn push_input(&mut self, channel: u64, val: i64) {
        let ch = channel as i64;
        self.vm
            .ether
            .entry(ch)
            .or_insert(VecDeque::new())
            .push_back(Value::Int(val));
        if let Some(queue) = self.vm.ether.get_mut(&ch) {
            if queue.len() > 5 {
                queue.pop_front();
            }
        }
    }

    fn sum_channel(&mut self, channel: u64) -> f32 {
        let ch = channel as i64;
        let mut sum = 0.0;
        if let Some(queue) = self.vm.ether.get_mut(&ch) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    sum += v as f32;
                }
            }
        }
        sum
    }

    // Flocking Helpers
    fn separation(&self, agents: &[Agent]) -> Vec4D {
        let mut steer = Vec4D::zero();
        let mut count = 0;
        for other in agents {
            if self.id == other.id {
                continue;
            }
            let d_sq = self.pos.distance_squared(other.pos);
            if d_sq > 0.0 && d_sq < self.dna_params.view_radius * self.dna_params.view_radius {
                let diff = (self.pos - other.pos).normalize().scale(1.0 / d_sq.sqrt());
                steer += diff;
                count += 1;
            }
        }
        if count > 0 {
            steer = steer.scale(1.0 / count as f32);
            if steer.length_squared() > 0.0 {
                steer = steer.normalize().scale(self.dna_params.max_speed) - self.vel;
                steer = steer.limit(self.dna_params.max_force);
            }
        }
        steer
    }

    fn alignment(&self, agents: &[Agent]) -> Vec4D {
        let mut sum = Vec4D::zero();
        let mut count = 0;
        for other in agents {
            if self.id == other.id {
                continue;
            }
            let d_sq = self.pos.distance_squared(other.pos);
            if d_sq > 0.0 && d_sq < self.dna_params.view_radius * self.dna_params.view_radius {
                sum += other.vel;
                count += 1;
            }
        }
        if count > 0 {
            sum = sum
                .scale(1.0 / count as f32)
                .normalize()
                .scale(self.dna_params.max_speed);
            let steer = sum - self.vel;
            return steer.limit(self.dna_params.max_force);
        }
        Vec4D::zero()
    }

    fn cohesion(&self, agents: &[Agent]) -> Vec4D {
        let mut sum = Vec4D::zero();
        let mut count = 0;
        for other in agents {
            if self.id == other.id {
                continue;
            }
            let d_sq = self.pos.distance_squared(other.pos);
            if d_sq > 0.0 && d_sq < self.dna_params.view_radius * self.dna_params.view_radius {
                sum += other.pos;
                count += 1;
            }
        }
        if count > 0 {
            sum = sum.scale(1.0 / count as f32);
            return self.seek(sum);
        }
        Vec4D::zero()
    }

    fn seek(&self, target: Vec4D) -> Vec4D {
        let desired = (target - self.pos)
            .normalize()
            .scale(self.dna_params.max_speed);
        let steer = desired - self.vel;
        steer.limit(self.dna_params.max_force)
    }

    fn bounce(&mut self, bounds: Vec4D) {
        if self.pos.x > bounds.x {
            self.pos.x = bounds.x;
            self.vel.x *= -1.0;
        } else if self.pos.x < -bounds.x {
            self.pos.x = -bounds.x;
            self.vel.x *= -1.0;
        }

        if self.pos.y > bounds.y {
            self.pos.y = bounds.y;
            self.vel.y *= -1.0;
        } else if self.pos.y < -bounds.y {
            self.pos.y = -bounds.y;
            self.vel.y *= -1.0;
        }

        if self.pos.z > bounds.z {
            self.pos.z = bounds.z;
            self.vel.z *= -1.0;
        } else if self.pos.z < -bounds.z {
            self.pos.z = -bounds.z;
            self.vel.z *= -1.0;
        }

        if self.pos.w > bounds.w {
            self.pos.w = bounds.w;
            self.vel.w *= -1.0;
        } else if self.pos.w < -bounds.w {
            self.pos.w = -bounds.w;
            self.vel.w *= -1.0;
        }
    }

    pub fn pollinate(&mut self, plant_color: Color) {
        // Blend colors (20%)
        let r = self.color.r * 0.8 + plant_color.r * 0.2;
        let g = self.color.g * 0.8 + plant_color.g * 0.2;
        let b = self.color.b * 0.8 + plant_color.b * 0.2;
        self.color = Color::new(r, g, b, 1.0);

        if ::rand::thread_rng().gen_bool(0.1) {
            self.dna_params.max_speed *= 1.01;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new_random(1);
        assert_eq!(agent.id, 1);
        assert!(agent.pos.length_squared() > 0.0);
    }

    #[test]
    fn test_agent_update_no_panic() {
        let mut agent = Agent::new_random(1);
        let agents = vec![agent.clone()];
        let monitor = SystemMonitor::new();
        let bounds = Vec4D::new(10.0, 10.0, 10.0, 10.0);

        agent.update(&agents, &monitor, None, bounds);
        agent.update(&agents, &monitor, None, bounds);
        assert!(agent.pos.length_squared() > 0.0);
    }
}
