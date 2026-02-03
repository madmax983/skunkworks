use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::{Directed, Graph};
use rand::prelude::*;
use std::collections::VecDeque;

pub type NodeId = NodeIndex;
pub type EdgeId = EdgeIndex;

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u64,
    pub source: NodeId,
    pub dest: NodeId,
    pub budget: f64,
    pub color: (u8, u8, u8),
    pub history: Vec<NodeId>,
}

impl Packet {
    pub fn new(id: u64, source: NodeId, dest: NodeId, budget: f64) -> Self {
        let mut rng = rand::thread_rng();
        let color = (
            rng.gen_range(100..255),
            rng.gen_range(100..255),
            rng.gen_range(200..255),
        );
        Self {
            id,
            source,
            dest,
            budget,
            color,
            history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    pub id: NodeId,
    pub queue: VecDeque<Packet>,
    pub balance: f64,
    pub base_price: f64,
    pub congestion_factor: f64,
    pub processing_rate: usize, // Packets per tick
    pub pos: (f64, f64),        // For UI
}

impl Router {
    pub fn new(id: NodeId, pos: (f64, f64)) -> Self {
        Self {
            id,
            queue: VecDeque::new(),
            balance: 0.0,
            base_price: 1.0,
            congestion_factor: 0.5,
            processing_rate: 1,
            pos,
        }
    }

    pub fn current_price(&self) -> f64 {
        self.base_price + (self.queue.len() as f64 * self.congestion_factor)
    }
}

#[derive(Debug, Clone)]
pub struct Link {
    pub packets: Vec<(Packet, f64)>, // Packet and progress (0.0 to 1.0)
    pub speed: f64,                  // Progress per tick
    pub length: f64,                 // UI distance (approx)
}

impl Link {
    pub fn new(speed: f64) -> Self {
        Self {
            packets: Vec::new(),
            speed,
            length: 1.0,
        }
    }
}

pub struct Network {
    pub graph: Graph<Router, Link, Directed>,
    pub dropped_packets: u64,
    pub total_packets: u64,
    pub tick_count: u64,
}

impl Network {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            dropped_packets: 0,
            total_packets: 0,
            tick_count: 0,
        }
    }

    pub fn generate_mesh(&mut self, width: usize, height: usize) {
        let mut indices = Vec::new();
        let mut rng = rand::thread_rng();

        // Create Nodes in Grid
        for y in 0..height {
            for x in 0..width {
                // Jitter position
                let jx = x as f64 * 10.0 + rng.gen_range(-2.0..2.0);
                let jy = y as f64 * 10.0 + rng.gen_range(-2.0..2.0);

                // Add dummy node first to get index
                let idx = self.graph.add_node(Router::new(NodeIndex::new(0), (jx, jy)));
                // Update ID
                self.graph[idx].id = idx;
                indices.push(idx);
            }
        }

        // Create Edges (Grid + Diagonals)
        for y in 0..height {
            for x in 0..width {
                let curr = y * width + x;
                let curr_idx = indices[curr];

                // East
                if x + 1 < width {
                    let next = y * width + (x + 1);
                    self.add_bidirectional_link(curr_idx, indices[next]);
                }
                // South
                if y + 1 < height {
                    let next = (y + 1) * width + x;
                    self.add_bidirectional_link(curr_idx, indices[next]);
                }
                // Random cross-links
                if rng.gen_bool(0.1) {
                     let random_idx = indices[rng.gen_range(0..indices.len())];
                     if random_idx != curr_idx {
                         self.add_bidirectional_link(curr_idx, random_idx);
                     }
                }
            }
        }
    }

    fn add_bidirectional_link(&mut self, a: NodeId, b: NodeId) {
        let mut rng = rand::thread_rng();
        // Random speed
        let speed = rng.gen_range(0.05..0.2);
        self.graph.add_edge(a, b, Link::new(speed));
        self.graph.add_edge(b, a, Link::new(speed));
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        let mut rng = rand::thread_rng();

        // 1. Move packets on links
        let mut arrival_events = Vec::new();
        for edge_idx in self.graph.edge_indices() {
             let (_source, target) = self.graph.edge_endpoints(edge_idx).unwrap();
             let link = &mut self.graph[edge_idx];

             let mut remaining = Vec::new();
             for (packet, mut progress) in link.packets.drain(..) {
                 progress += link.speed;
                 if progress >= 1.0 {
                     arrival_events.push((target, packet));
                 } else {
                     remaining.push((packet, progress));
                 }
             }
             link.packets = remaining;
        }

        // Place arrived packets into queues
        for (node_idx, packet) in arrival_events {
            // Check if arrived at final destination
            if packet.dest == node_idx {
                // Success! (Consumed)
                // Maybe add to stats?
            } else {
                self.graph[node_idx].queue.push_back(packet);
            }
        }

        // 2. Spawn new packets (Traffic Generation)
        if self.tick_count % 5 == 0 {
            let node_count = self.graph.node_count();
            if node_count > 1 {
                let src = NodeIndex::new(rng.gen_range(0..node_count));
                let dst = NodeIndex::new(rng.gen_range(0..node_count));
                if src != dst {
                    let budget = rng.gen_range(50.0..150.0); // Random budget
                    self.total_packets += 1;
                    let p = Packet::new(self.total_packets, src, dst, budget);
                    self.graph[src].queue.push_back(p);
                }
            }
        }

        // 3. Router Decision Phase (The Market)
        // Snapshot prices first to ensure synchronous decisions
        let prices: Vec<(NodeId, f64)> = self.graph.node_indices()
            .map(|idx| (idx, self.graph[idx].current_price()))
            .collect();
        // Map for quick lookup
        let price_map: std::collections::HashMap<NodeId, f64> = prices.iter().cloned().collect();

        // Collect moves to execute
        let mut moves: Vec<(EdgeId, Packet, f64)> = Vec::new(); // (Edge, Packet, CostPaid)

        for node_idx in self.graph.node_indices() {
            let mut packets_to_route = Vec::new();

            // Scope for borrowing graph node
            {
                let router = &mut self.graph[node_idx];
                let rate = router.processing_rate;

                // Take up to 'rate' packets
                for _ in 0..rate {
                    if let Some(p) = router.queue.pop_front() {
                        packets_to_route.push(p);
                    }
                }
            }

            // Route them
            for mut p in packets_to_route {
                // Find neighbors
                let neighbors: Vec<(NodeId, EdgeId)> = self.graph.neighbors(node_idx)
                    .map(|n| {
                        let e = self.graph.find_edge(node_idx, n).unwrap();
                        (n, e)
                    })
                    .collect();

                if neighbors.is_empty() {
                    // Dead end, drop
                    self.dropped_packets += 1;
                    continue;
                }

                // Decision Logic: Min(Price + Heuristic)
                // Heuristic: Distance to dest.
                // Since we don't have geo-distance easily without lookup, let's use Euclidean dist logic if we can access positions.
                // We can access 'dest' position if we lookup.

                let dest_pos = self.graph[p.dest].pos;

                let mut best_edge = None;
                let mut min_cost = f64::MAX;
                let mut best_price = 0.0;

                for (n_idx, e_idx) in neighbors {
                    let n_price = *price_map.get(&n_idx).unwrap_or(&1000.0);
                    let n_pos = self.graph[n_idx].pos;

                    // Dist heuristic
                    let dx = n_pos.0 - dest_pos.0;
                    let dy = n_pos.1 - dest_pos.1;
                    let dist = (dx*dx + dy*dy).sqrt();

                    // Total perceived cost
                    let total_cost = n_price + (dist * 0.1); // Weight distance less than price?

                    if total_cost < min_cost {
                        min_cost = total_cost;
                        best_edge = Some(e_idx);
                        best_price = n_price;
                    }
                }

                if let Some(edge) = best_edge {
                    if p.budget >= best_price {
                        p.budget -= best_price;
                        p.history.push(node_idx);
                        moves.push((edge, p, best_price));
                    } else {
                        // Bankrupt
                        self.dropped_packets += 1;
                    }
                } else {
                    // trapped
                     self.dropped_packets += 1;
                }
            }
        }

        // Apply moves
        for (edge_idx, packet, cost) in moves {
            // Credit the router? (The target router gets the money? Or the link provider?)
            // In this model, you pay to *enter* the next router.
            // So we need to credit the target node.
            let (_, target) = self.graph.edge_endpoints(edge_idx).unwrap();
            self.graph[target].balance += cost;

            self.graph[edge_idx].packets.push((packet, 0.0));
        }
    }

    pub fn burst(&mut self, amount: usize) {
         let mut rng = rand::thread_rng();
         let node_count = self.graph.node_count();
         if node_count < 2 { return; }

         for _ in 0..amount {
            let src = NodeIndex::new(rng.gen_range(0..node_count));
            let dst = NodeIndex::new(rng.gen_range(0..node_count));
            if src != dst {
                 let budget = rng.gen_range(100.0..500.0); // Richer packets for burst
                 self.total_packets += 1;
                 let p = Packet::new(self.total_packets, src, dst, budget);
                 self.graph[src].queue.push_back(p);
            }
         }
    }
}
