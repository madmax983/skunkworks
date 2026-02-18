use macroquad::prelude::*;
use ::rand::prelude::*;

#[derive(Clone, Copy)]
pub struct ServiceNode {
    pub pos: Vec2,
    pub load: f32,      // 0.0 to 1.0 (Current traffic)
    pub capacity: f32,  // Max load before stress
    pub health: f32,    // 0.0 to 1.0 (1.0 = Healthy)
    pub radius: f32,
    pub is_gateway: bool,
}

impl ServiceNode {
    pub fn new(pos: Vec2, is_gateway: bool) -> Self {
        Self {
            pos,
            load: 0.0,
            capacity: 100.0,
            health: 1.0,
            radius: if is_gateway { 15.0 } else { 8.0 },
            is_gateway,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Hypha {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
    pub flow_rate: f32,
    pub parent_index: Option<usize>, // Index in hyphae vector
}

pub struct TrafficParticle {
    pub pos: Vec2,
    pub target: Vec2,
    pub speed: f32,
    pub active: bool,
}

pub struct World {
    pub nodes: Vec<ServiceNode>,
    pub hyphae: Vec<Hypha>,
    pub particles: Vec<TrafficParticle>,
    pub width: f32,
    pub height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        let mut world = Self {
            nodes: Vec::new(),
            hyphae: Vec::new(),
            particles: Vec::new(),
            width,
            height,
        };

        // Create a Gateway node in the center
        let center = vec2(width / 2.0, height / 2.0);
        world.nodes.push(ServiceNode::new(center, true));

        world
    }

    pub fn update(&mut self, dt: f32) {
        let mut rng = thread_rng();

        // 1. Node Metabolism
        for node in &mut self.nodes {
            // Load decays over time (processed requests)
            node.load = (node.load - dt * 20.0).max(0.0);

            // Health regeneration/decay
            if node.load > node.capacity {
                node.health -= dt * 0.5; // Overload damage
            } else {
                node.health = (node.health + dt * 0.1).min(1.0);
            }
        }

        // 2. Hyphal Growth (Simple MST-like behavior)
        // Connect unconnected nodes to the nearest existing structure (Node or Hypha endpoint)
        let mut new_hyphae = Vec::new();

        // Collect all potential connection points (Nodes + Hyphae ends)
        let mut connection_points: Vec<Vec2> = Vec::new();
        for node in &self.nodes {
            if node.is_gateway {
                connection_points.push(node.pos);
            }
        }
        for hypha in &self.hyphae {
            connection_points.push(hypha.end);
        }

        // For each node, if it's not connected, try to connect it
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_gateway { continue; }

            let mut connected = false;
            for hypha in &self.hyphae {
                if hypha.end.distance(node.pos) < node.radius + 2.0 {
                    connected = true;
                    break;
                }
            }

            if !connected {
                // Find nearest connection point
                let mut nearest_pt = Vec2::ZERO;
                let mut min_dist = f32::MAX;

                for pt in &connection_points {
                    let d = pt.distance(node.pos);
                    if d < min_dist {
                        min_dist = d;
                        nearest_pt = *pt;
                    }
                }

                // Grow towards it
                if min_dist < f32::MAX {
                    let dir = (node.pos - nearest_pt).normalize_or_zero();
                    let growth_speed = 100.0 * dt; // Pixels per second

                    let new_end = nearest_pt + dir * growth_speed.min(min_dist);

                    new_hyphae.push(Hypha {
                        start: nearest_pt,
                        end: new_end,
                        width: 2.0,
                        flow_rate: 0.0,
                        parent_index: None, // Simplified for now
                    });
                }
            }
        }
        self.hyphae.extend(new_hyphae);

        // 3. Traffic Simulation
        // Spawn particles at Gateway if there is load
        if rng.gen_bool(0.1) {
             let mut gateway_pos = None;
             let mut should_spawn = false;

             if let Some(gateway) = self.nodes.iter().find(|n| n.is_gateway) {
                 if gateway.load > 10.0 {
                     gateway_pos = Some(gateway.pos);
                     should_spawn = true;
                 }
             }

             if should_spawn && self.nodes.len() > 1 {
                 if let Some(start_pos) = gateway_pos {
                     let target_idx = rng.gen_range(1..self.nodes.len());
                     let target = self.nodes[target_idx].pos;
                     self.particles.push(TrafficParticle {
                         pos: start_pos,
                         target,
                         speed: 200.0,
                         active: true,
                     });
                 }
             }
        }

        // Move particles
        for p in &mut self.particles {
            if !p.active { continue; }
            let dir = (p.target - p.pos).normalize_or_zero();
            p.pos += dir * p.speed * dt;

            if p.pos.distance(p.target) < 5.0 {
                p.active = false;
            }
        }

        // Apply load from arrived particles
        // (Clean up dead particles)
        let mut arrived_targets = Vec::new();
        self.particles.retain(|p| {
            if !p.active {
                arrived_targets.push(p.target);
                false
            } else {
                true
            }
        });

        for target_pos in arrived_targets {
            for node in &mut self.nodes {
                if node.pos.distance(target_pos) < node.radius {
                    node.load += 5.0;
                }
            }
        }

        // 4. Autoscaling (Spawn new nodes if system is stressed)
        let total_load: f32 = self.nodes.iter().map(|n| n.load).sum();
        let avg_load = if self.nodes.is_empty() { 0.0 } else { total_load / self.nodes.len() as f32 };

        // Find stressed node position without holding a borrow
        let stressed_pos = if avg_load > 50.0 && self.nodes.len() < 100 && rng.gen_bool(0.05) {
            self.nodes.iter()
                .max_by_key(|n| (n.load * 100.0) as i32)
                .map(|n| n.pos)
        } else {
            None
        };

        if let Some(pos) = stressed_pos {
             let angle = rng.gen_range(0.0..std::f32::consts::TAU);
             let dist = rng.gen_range(50.0..150.0);
             let offset = vec2(angle.cos(), angle.sin()) * dist;
             let new_pos = pos + offset;

             // Keep in bounds
             let new_pos = new_pos.clamp(vec2(10.0, 10.0), vec2(self.width - 10.0, self.height - 10.0));

             self.nodes.push(ServiceNode::new(new_pos, false));
        }
    }

    pub fn spawn_node(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(50.0..self.width - 50.0);
        let y = rng.gen_range(50.0..self.height - 50.0);
        self.nodes.push(ServiceNode::new(vec2(x, y), false));
    }

    pub fn add_load_at(&mut self, pos: Vec2) {
        // Find nearest node
        let mut nearest = None;
        let mut min_dist = f32::MAX;

        for node in &mut self.nodes {
            let d = node.pos.distance(pos);
            if d < min_dist {
                min_dist = d;
                nearest = Some(node);
            }
        }

        if let Some(node) = nearest {
            if min_dist < 100.0 {
                node.load += 20.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metabolism() {
        let mut world = World::new(100.0, 100.0);
        // Gateway is at index 0
        world.nodes[0].load = 50.0;

        world.update(0.1);

        // Load should decay: 50.0 - 0.1 * 20.0 = 48.0
        assert!(world.nodes[0].load < 50.0);
    }
}
