use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Mushroom {
    pub pos: Vec2,
    pub load: f32,
    pub capacity: f32,
    pub connections: Vec<usize>,
}

impl Mushroom {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            load: 0.0,
            capacity: 100.0,
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub pos: Vec2,
    pub target_index: usize,
    pub payload: f32,
    pub velocity: Vec2,
}

#[derive(Clone, Debug)]
pub struct Hypha {
    pub from: usize,
    pub to: usize,
    pub flow: f32,
}

pub struct World {
    pub mushrooms: Vec<Mushroom>,
    pub packets: Vec<Packet>,
    pub hyphae: Vec<Hypha>,
    pub width: f32,
    pub height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            mushrooms: Vec::new(),
            packets: Vec::new(),
            hyphae: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_mushroom(&mut self, pos: Vec2) -> usize {
        let index = self.mushrooms.len();
        self.mushrooms.push(Mushroom::new(pos));

        // Connect to nearby mushrooms (simple distance based connection for now)
        // In a real moonshot, this would be dynamic growth.
        // For now, let's just connect to the nearest 2 neighbors to form a graph.
        let mut distances: Vec<(usize, f32)> = self
            .mushrooms
            .iter()
            .enumerate()
            .take(index) // Don't connect to self
            .map(|(i, m)| (i, m.pos.distance(pos)))
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (neighbor_idx, dist) in distances.iter().take(3) {
            if *dist < 300.0 {
                self.connect(index, *neighbor_idx);
            }
        }

        index
    }

    pub fn connect(&mut self, i: usize, j: usize) {
        if !self.mushrooms[i].connections.contains(&j) {
            self.mushrooms[i].connections.push(j);
            self.mushrooms[j].connections.push(i);
            self.hyphae.push(Hypha {
                from: i,
                to: j,
                flow: 0.0,
            });
        }
    }

    pub fn update(&mut self, dt: f32) {
        // 1. Packet Movement
        let mut dead_packets = Vec::new();
        for (i, packet) in self.packets.iter_mut().enumerate() {
            let target_pos = self.mushrooms[packet.target_index].pos;
            let dir = (target_pos - packet.pos).normalize_or_zero();
            packet.pos += dir * 200.0 * dt; // Speed 200

            if packet.pos.distance(target_pos) < 5.0 {
                // Arrived
                self.mushrooms[packet.target_index].load += packet.payload;
                dead_packets.push(i);
            }
        }

        // Remove dead packets (reverse order to keep indices valid)
        for i in dead_packets.into_iter().rev() {
            self.packets.remove(i);
        }

        // 2. Load Calculation (Pass 1: Determine excess)
        // We collect (source_index, target_index, amount)
        let mut transfers = Vec::new();

        for i in 0..self.mushrooms.len() {
            // Processing: Load naturally decays (processing requests)
            let decay = 10.0 * dt;
            if self.mushrooms[i].load > 0.0 {
                self.mushrooms[i].load = (self.mushrooms[i].load - decay).max(0.0);
            }

            // Balancing: If overloaded, send packet
            if self.mushrooms[i].load > self.mushrooms[i].capacity {
                let excess = self.mushrooms[i].load - self.mushrooms[i].capacity;
                let transfer_amount = excess * 5.0 * dt; // Transfer 5x excess per second

                // Find best neighbor (lowest load)
                let mut best_neighbor = None;
                let mut min_load = f32::MAX;

                for &neighbor_idx in &self.mushrooms[i].connections {
                    let neighbor_load = self.mushrooms[neighbor_idx].load;
                    if neighbor_load < self.mushrooms[neighbor_idx].capacity
                        && neighbor_load < min_load
                    {
                        min_load = neighbor_load;
                        best_neighbor = Some(neighbor_idx);
                    }
                }

                if let Some(target_idx) = best_neighbor {
                    transfers.push((i, target_idx, transfer_amount));
                }
            }
        }

        // Pass 2: Apply Transfers
        for (source_idx, target_idx, amount) in transfers {
            self.mushrooms[source_idx].load -= amount;
            self.packets.push(Packet {
                pos: self.mushrooms[source_idx].pos,
                target_index: target_idx,
                payload: amount,
                velocity: Vec2::ZERO,
            });
        }
    }
}
