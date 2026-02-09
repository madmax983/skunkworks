use macroquad::prelude::*;

const MAX_LOAD: f32 = 100.0;
const SPAWN_RATE: f32 = 0.8; // Probability per frame
const SPEED: f32 = 4.0;
const LOCUST_TTL: f32 = 600.0; // Frames

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeType {
    Normal,
    Spawner,
    Target,
    Firewall,
}

#[derive(Debug, Clone)]
struct Node {
    pos: Vec2,
    load: f32,
    capacity: f32,
    node_type: NodeType,
    is_crashed: bool,
}

impl Node {
    fn new(pos: Vec2, node_type: NodeType) -> Self {
        Self {
            pos,
            load: 0.0,
            capacity: MAX_LOAD,
            node_type,
            is_crashed: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Edge {
    start: usize,
    end: usize,
    bandwidth: f32,
    congestion: f32,
}

#[derive(Debug, Clone)]
struct Locust {
    pos: Vec2,
    vel: Vec2,
    target_node: Option<usize>,
    ttl: f32,
    state: LocustState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LocustState {
    Seeking,
    Processing, // Arrived at node, waiting to forward
    Dead,
    Success,
}

struct Network {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    locusts: Vec<Locust>,
}

impl Network {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            locusts: Vec::new(),
        }
    }

    fn update(&mut self) {
        // Spawn Logic
        // We can't iterate nodes and mutate locusts easily if we use `self` methods.
        // Let's gather spawn locations first.
        let spawn_positions: Vec<Vec2> = self.nodes.iter()
            .filter(|n| n.node_type == NodeType::Spawner)
            .map(|n| n.pos)
            .collect();

        for pos in spawn_positions {
            if rand::gen_range(0.0, 1.0) < SPAWN_RATE {
                self.locusts.push(Locust {
                    pos,
                    vel: vec2(0.0, 0.0),
                    target_node: None,
                    ttl: LOCUST_TTL,
                    state: LocustState::Seeking,
                });
            }
        }

        // Target position for heuristic
        let target_pos = self.nodes.iter().find(|n| n.node_type == NodeType::Target).map(|n| n.pos);

        // Update Locusts
        // We need to access nodes/edges while mutating locusts.
        // We can't do this with methods on self.
        // We must split the borrows.
        let nodes = &mut self.nodes;
        let edges = &self.edges;
        let locusts = &mut self.locusts;

        for locust in locusts.iter_mut() {
            locust.ttl -= 1.0;
            if locust.ttl <= 0.0 {
                locust.state = LocustState::Dead;
            }

            if locust.state == LocustState::Dead || locust.state == LocustState::Success {
                continue;
            }

            // Pick target if none
            if locust.target_node.is_none() {
                let current_node_idx = Self::find_nearest_node(nodes, locust.pos);

                // Pick a neighbor
                if let Some(next_hop) = Self::pick_next_hop(nodes, edges, current_node_idx, target_pos) {
                    locust.target_node = Some(next_hop);
                } else {
                    locust.state = LocustState::Dead;
                }
            }

            // Move
            if let Some(target_idx) = locust.target_node {
                let target_node_pos = nodes[target_idx].pos; // Copy pos to avoid borrow issues? Vec2 is Copy.

                let dir = (target_node_pos - locust.pos).normalize_or_zero();
                locust.vel = dir * SPEED;
                locust.pos += locust.vel;

                // Check arrival
                if locust.pos.distance(target_node_pos) < 5.0 {
                    // Arrived
                    // Interaction with Node
                    let node = &mut nodes[target_idx];
                    node.load += 1.0;

                    if node.node_type == NodeType::Target {
                        locust.state = LocustState::Success;
                    } else if node.node_type == NodeType::Firewall {
                         locust.state = LocustState::Dead;
                    } else if node.is_crashed {
                         // Node is down, packet dropped
                         locust.state = LocustState::Dead;
                    } else {
                        // Continue routing
                        locust.target_node = None; // Will pick next hop next frame
                        // Force position to node center to prevent "orbiting"
                        locust.pos = node.pos;
                    }
                }
            }
        }

        // Remove dead
        self.locusts.retain(|l| l.state != LocustState::Dead && l.state != LocustState::Success);

        // Node Logic (Crash/Decay)
        for node in &mut self.nodes {
            // Decay
            node.load = (node.load - 0.5).max(0.0);

            // Crash check
            if node.load > node.capacity {
                node.is_crashed = true;
            } else if node.load < node.capacity * 0.5 {
                node.is_crashed = false; // Recovery
            }
        }
    }

    // Static helper methods to avoid capturing `self`
    fn find_nearest_node(nodes: &[Node], pos: Vec2) -> usize {
        let mut min_dist = f32::MAX;
        let mut idx = 0;
        for (i, node) in nodes.iter().enumerate() {
            let d = pos.distance(node.pos);
            if d < min_dist {
                min_dist = d;
                idx = i;
            }
        }
        idx
    }

    fn pick_next_hop(nodes: &[Node], edges: &[Edge], current_node: usize, target_pos: Option<Vec2>) -> Option<usize> {
        let mut candidates = Vec::new();
        for edge in edges {
            if edge.start == current_node {
                candidates.push(edge.end);
            } else if edge.end == current_node {
                candidates.push(edge.start);
            }
        }

        if candidates.is_empty() {
            return None;
        }

        candidates.sort_by(|&a, &b| {
            let node_a = &nodes[a];
            let node_b = &nodes[b];

            let dist_a = target_pos.map_or(0.0, |t| node_a.pos.distance(t));
            let dist_b = target_pos.map_or(0.0, |t| node_b.pos.distance(t));

            // Heuristic: Cost = Load + Distance
            // High load = Bad. High distance = Bad.
            let cost_a = node_a.load + dist_a * 0.2;
            let cost_b = node_b.load + dist_b * 0.2;

            cost_a.partial_cmp(&cost_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Add some noise to prevent single-file lines
        // Use macroquad's rand
        let range = candidates.len().min(2);
        let pick = rand::gen_range(0, range);
        Some(candidates[pick])
    }
}

// ... main ...
#[macroquad::main("Packet Locusts")]
async fn main() {
    let mut network = Network::new();

    // Initialize map
    // Spawner
    network.nodes.push(Node::new(vec2(100.0, 300.0), NodeType::Spawner)); // 0

    // Grid of Routers
    let rows = 5;
    let cols = 8;
    let spacing = 80.0;
    let offset_x = 200.0;
    let offset_y = 100.0;

    for y in 0..rows {
        for x in 0..cols {
            network.nodes.push(Node::new(
                vec2(offset_x + x as f32 * spacing, offset_y + y as f32 * spacing),
                NodeType::Normal
            ));
        }
    }

    // Target
    let target_idx = network.nodes.len();
    network.nodes.push(Node::new(vec2(offset_x + cols as f32 * spacing + 50.0, 300.0), NodeType::Target));

    // Connect Spawner to first column
    for y in 0..rows {
        let idx = 1 + y * cols; // First column
        network.edges.push(Edge { start: 0, end: idx, bandwidth: 5.0, congestion: 0.0 });
    }

    // Connect Grid (Right and Down and Diagonals for better flow)
    for y in 0..rows {
        for x in 0..cols {
            let current = 1 + y * cols + x;
            // Right
            if x < cols - 1 {
                let next = current + 1;
                network.edges.push(Edge { start: current, end: next, bandwidth: 5.0, congestion: 0.0 });
            }
            // Down
            if y < rows - 1 {
                let down = current + cols;
                network.edges.push(Edge { start: current, end: down, bandwidth: 5.0, congestion: 0.0 });
            }
            // Diagonal Down-Right (to prevent grid lock)
             if x < cols - 1 && y < rows - 1 {
                let diag = current + cols + 1;
                network.edges.push(Edge { start: current, end: diag, bandwidth: 5.0, congestion: 0.0 });
            }
        }
    }

    // Connect last column to Target
    for y in 0..rows {
        let idx = 1 + y * cols + (cols - 1);
        network.edges.push(Edge { start: idx, end: target_idx, bandwidth: 5.0, congestion: 0.0 });
    }

    loop {
        clear_background(BLACK);

        network.update();

        // Draw
        for edge in &network.edges {
            let start = network.nodes[edge.start].pos;
            let end = network.nodes[edge.end].pos;
            draw_line(start.x, start.y, end.x, end.y, 1.0, DARKGRAY);
        }

        for node in &network.nodes {
            let color = match node.node_type {
                NodeType::Spawner => BLUE,
                NodeType::Target => GREEN,
                NodeType::Firewall => MAGENTA,
                NodeType::Normal => {
                    if node.is_crashed { RED }
                    else {
                        // Lerp color based on load
                        let t = (node.load / node.capacity).clamp(0.0, 1.0);
                        // Green (Low) -> Yellow -> Red (High)
                        if t < 0.5 {
                            Color::new(t * 2.0, 1.0, 0.0, 1.0)
                        } else {
                            Color::new(1.0, (1.0 - t) * 2.0, 0.0, 1.0)
                        }
                    }
                }
            };
            draw_circle(node.pos.x, node.pos.y, 8.0, color);
            // Draw load bar
            let load_ratio = node.load / node.capacity;
            draw_rectangle(node.pos.x - 10.0, node.pos.y - 15.0, 20.0 * load_ratio, 3.0, color);
        }

        for locust in &network.locusts {
            draw_poly(locust.pos.x, locust.pos.y, 3, 3.0, 0.0, ORANGE);
        }

        draw_text(&format!("Locusts: {}", network.locusts.len()), 20.0, 30.0, 20.0, WHITE);
        draw_text("Packet Locusts: Simulation", 20.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
