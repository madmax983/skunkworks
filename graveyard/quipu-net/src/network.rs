use rand::prelude::*;
use crate::quipu::{Quipu, Pendant, KnotType};
use crate::serializer::to_quipu;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub id: u64,
    pub source: usize,
    pub dest: usize,
    pub payload: Quipu,
    pub progress: f64, // 0.0 to 1.0
    pub speed: f64,
    pub message: String,
}

#[derive(Serialize)]
struct MessageData {
    id: u64,
    content: u64, // Represents some data
    timestamp: u64,
}

pub struct Simulation {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub packets: Vec<Packet>,
    pub rng: ThreadRng,
    pub packet_counter: u64,
}

impl Simulation {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Create Nodes in a circle or random layout
        let num_nodes = 8;
        for i in 0..num_nodes {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / (num_nodes as f64);
            let radius = 0.4;
            let x = 0.5 + radius * angle.cos();
            let y = 0.5 + radius * angle.sin();
            nodes.push(Node {
                id: i,
                x,
                y,
                name: format!("Node-{}", i),
            });
        }

        // Create Edges (Cycle + Random chords)
        for i in 0..num_nodes {
            edges.push(Edge {
                source: i,
                target: (i + 1) % num_nodes,
            });
            // Random chord
            if rng.gen_bool(0.3) {
                let target = rng.gen_range(0..num_nodes);
                if target != i && target != (i + 1) % num_nodes {
                    edges.push(Edge { source: i, target });
                }
            }
        }

        Self {
            nodes,
            edges,
            packets: Vec::new(),
            rng,
            packet_counter: 0,
        }
    }

    pub fn spawn_packet(&mut self) {
        if self.edges.is_empty() { return; }

        // Pick a random edge
        let edge = &self.edges[self.rng.gen_range(0..self.edges.len())];

        self.packet_counter += 1;
        let data = MessageData {
            id: self.packet_counter,
            content: self.rng.gen_range(100..10000), // Random data
            timestamp: 1000 + self.packet_counter,
        };

        // Serialize to Quipu
        let payload = to_quipu(&data).unwrap_or_else(|_| Quipu::new());

        // Calculate speed based on complexity
        let complexity = calculate_complexity(&payload);
        // Base speed = 0.02 per tick.
        // Heavier quipu = slower.
        // complexity is roughly number of knots + value weights.
        // Let's say speed = 0.05 / (1.0 + complexity * 0.01)
        let speed = 0.05 / (1.0 + (complexity as f64) * 0.05);

        self.packets.push(Packet {
            id: self.packet_counter,
            source: edge.source,
            dest: edge.target,
            payload,
            progress: 0.0,
            speed,
            message: format!("MSG #{}", data.id),
        });
    }

    pub fn update(&mut self) {
        // Spawn packets randomly
        if self.rng.gen_bool(0.05) {
            self.spawn_packet();
        }

        // Move packets
        let mut arrived = Vec::new();
        for (i, packet) in self.packets.iter_mut().enumerate() {
            packet.progress += packet.speed;
            if packet.progress >= 1.0 {
                arrived.push(i);
            }
        }

        // Remove arrived packets (in reverse order to keep indices valid)
        for i in arrived.into_iter().rev() {
            self.packets.remove(i);
        }
    }
}

fn calculate_complexity(quipu: &Quipu) -> u32 {
    let mut score = 0;
    for pendant in &quipu.pendants {
        score += pendant_complexity(pendant);
    }
    score
}

fn pendant_complexity(pendant: &Pendant) -> u32 {
    let mut score = 0;
    for knot in &pendant.knots {
        match knot.knot_type {
            KnotType::Single => score += 1,
            KnotType::Long(turns) => score += turns as u32,
            KnotType::FigureEight => score += 2, // Figure eight is complex
            KnotType::Empty => {},
        }
    }
    for sub in &pendant.subsidiaries {
        score += pendant_complexity(sub);
    }
    score
}
