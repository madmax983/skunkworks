use rand::Rng;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LinkId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PacketId(pub usize);

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: PacketId,
    pub source: NodeId,
    pub destination: NodeId,
    pub current_link: Option<LinkId>,
    pub t: f32, // Progress on link (0.0 to 1.0)
    pub budget: f32,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub id: LinkId,
    pub from: NodeId,
    pub to: NodeId,
    pub capacity: usize,           // Max concurrent packets (or bandwidth)
    pub queue: VecDeque<PacketId>, // Packets waiting to enter
    pub on_link: Vec<PacketId>,    // Packets currently traversing
    pub base_cost: f32,
    pub current_cost: f32,
    pub length: f32,
}

impl Link {
    pub fn update_cost(&mut self) {
        let load = (self.queue.len() + self.on_link.len()) as f32;
        let cap = self.capacity as f32;
        // Cost increases quadratically with load to discourage congestion
        // Avoid division by zero
        let cap = if cap < 1.0 { 1.0 } else { cap };
        self.current_cost = self.base_cost * (1.0 + (load / cap).powi(2));
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub pos: (f32, f32),
    // Routing Table: Destination -> (Next Hop Link, Total Cost)
    pub routing_table: HashMap<NodeId, (LinkId, f32)>,
}

pub struct World {
    pub nodes: HashMap<NodeId, Node>,
    pub links: HashMap<LinkId, Link>,
    pub packets: HashMap<PacketId, Packet>,
    pub next_packet_id: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            packets: HashMap::new(),
            next_packet_id: 0,
        }
    }

    pub fn add_node(&mut self, id: usize, pos: (f32, f32)) {
        self.nodes.insert(
            NodeId(id),
            Node {
                id: NodeId(id),
                pos,
                routing_table: HashMap::new(),
            },
        );
    }

    pub fn add_link(
        &mut self,
        id: usize,
        from: usize,
        to: usize,
        capacity: usize,
        base_cost: f32,
        length: f32,
    ) {
        self.links.insert(
            LinkId(id),
            Link {
                id: LinkId(id),
                from: NodeId(from),
                to: NodeId(to),
                capacity,
                queue: VecDeque::new(),
                on_link: Vec::new(),
                base_cost,
                current_cost: base_cost,
                length,
            },
        );
    }

    pub fn update_routes(&mut self) {
        let mut old_costs: HashMap<NodeId, HashMap<NodeId, f32>> = HashMap::new();
        for (id, node) in &self.nodes {
            let costs = node
                .routing_table
                .iter()
                .map(|(dest, (_, cost))| (*dest, *cost))
                .collect();
            old_costs.insert(*id, costs);
        }

        let mut outgoing: HashMap<NodeId, Vec<LinkId>> = HashMap::new();
        for link in self.links.values() {
            outgoing.entry(link.from).or_default().push(link.id);
        }

        for (node_id, node) in self.nodes.iter_mut() {
            let mut new_table: HashMap<NodeId, (LinkId, f32)> = HashMap::new();

            if let Some(links) = outgoing.get(node_id) {
                for &link_id in links {
                    let link = self.links.get(&link_id).unwrap();
                    let neighbor = link.to;
                    let link_cost = link.current_cost;

                    let cost_to_neighbor = link_cost;

                    let current_best_to_neighbor = new_table
                        .get(&neighbor)
                        .map(|(_, c)| *c)
                        .unwrap_or(f32::MAX);
                    if cost_to_neighbor < current_best_to_neighbor {
                        new_table.insert(neighbor, (link_id, cost_to_neighbor));
                    }

                    if let Some(neighbor_costs) = old_costs.get(&neighbor) {
                        for (&dest, &cost_from_neighbor) in neighbor_costs {
                            if dest == *node_id {
                                continue;
                            }

                            let total_cost = link_cost + cost_from_neighbor;
                            let current_best =
                                new_table.get(&dest).map(|(_, c)| *c).unwrap_or(f32::MAX);

                            if total_cost < current_best {
                                new_table.insert(dest, (link_id, total_cost));
                            }
                        }
                    }
                }
            }
            node.routing_table = new_table;
        }
    }

    pub fn step(&mut self) {
        // 1. Update Routes (every tick for now)
        self.update_routes();

        // 2. Update Link Costs
        // We do this before moving packets so routing is based on last tick's state, but cost is fresh?
        // Actually, update_routes uses current_cost. So costs should be updated before routes?
        // But let's stick to the plan: Routes -> Move -> Costs?
        // Logic:
        // - Packets move based on Routes.
        // - Routes are calculated based on Costs.
        // - Costs are calculated based on Load (Packets).
        // Cycle: Load -> Cost -> Routes -> Movement -> Load.
        // So:
        // 1. Update Costs (from current Load).
        // 2. Update Routes (from current Costs).
        // 3. Move Packets (using Routes).
        // 4. (Next tick) Load has changed.

        for link in self.links.values_mut() {
            link.update_cost();
        }

        // Use destructuring to split borrows
        let World {
            nodes,
            links,
            packets,
            next_packet_id,
        } = self;

        // 3. Move Packets on Links
        let mut transitions: Vec<(PacketId, NodeId)> = Vec::new(); // Packet, ArrivedAtNode
        let mut finished_packets: Vec<PacketId> = Vec::new();

        for link in links.values_mut() {
            let mut i = 0;
            while i < link.on_link.len() {
                let pid = link.on_link[i];
                if let Some(packet) = packets.get_mut(&pid) {
                    // Constant speed for now, or based on link length?
                    // Let's say speed = 1.0 / length. If length is 100.0, speed is 0.01.
                    // Min speed 0.01
                    let speed = if link.length > 0.0 {
                        1.0 / link.length
                    } else {
                        1.0
                    };
                    packet.t += speed;

                    if packet.t >= 1.0 {
                        // Reached end of link
                        link.on_link.remove(i);
                        transitions.push((pid, link.to));
                        // Don't increment i, as we removed element
                    } else {
                        i += 1;
                    }
                } else {
                    // Packet not found? Should not happen. Remove phantom ID.
                    link.on_link.remove(i);
                }
            }
        }

        // 4. Handle Transitions (Arrivals or Routing)
        for (pid, node_id) in transitions {
            let packet = packets.get_mut(&pid).unwrap();

            if node_id == packet.destination {
                // Arrived!
                finished_packets.push(pid);
                packet.current_link = None;
            } else {
                // Needs routing
                if let Some(node) = nodes.get(&node_id) {
                    if let Some((next_link_id, _)) = node.routing_table.get(&packet.destination) {
                        if let Some(link) = links.get_mut(next_link_id) {
                            link.queue.push_back(pid);
                            packet.current_link = Some(*next_link_id);
                            packet.t = 0.0;
                        } else {
                            // Link routing points to doesn't exist? Drop packet.
                            finished_packets.push(pid);
                        }
                    } else {
                        // No route found? Drop packet.
                        finished_packets.push(pid);
                    }
                } else {
                    // Node doesn't exist? Drop.
                    finished_packets.push(pid);
                }
            }
        }

        // Clean up finished packets
        for pid in finished_packets {
            packets.remove(&pid);
        }

        // 5. Process Queues (Move from Queue to OnLink)
        for link in links.values_mut() {
            while link.on_link.len() < link.capacity && !link.queue.is_empty() {
                if let Some(pid) = link.queue.pop_front() {
                    link.on_link.push(pid);
                }
            }
        }

        // 6. Spawn New Packets
        // Simple random spawner
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.2 {
            // 20% chance per tick to spawn a packet
            // Pick random source and dest
            let node_ids: Vec<NodeId> = nodes.keys().cloned().collect();
            if node_ids.len() >= 2 {
                let source = node_ids[rng.gen_range(0..node_ids.len())];
                let dest = node_ids[rng.gen_range(0..node_ids.len())];

                if source != dest {
                    // Check if source has a route
                    let mut routed = false;
                    let mut start_link = None;

                    if let Some(node) = nodes.get(&source) {
                        if let Some((link_id, _)) = node.routing_table.get(&dest) {
                            start_link = Some(link_id);
                            routed = true;
                        }
                    }

                    if routed {
                        let pid = PacketId(*next_packet_id);
                        *next_packet_id += 1;

                        let packet = Packet {
                            id: pid,
                            source,
                            destination: dest,
                            current_link: Some(*start_link.unwrap()),
                            t: 0.0,
                            budget: 100.0,
                        };

                        packets.insert(pid, packet);

                        // Add to link queue
                        if let Some(link) = links.get_mut(start_link.unwrap()) {
                            link.queue.push_back(pid);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_preference() {
        let mut world = World::new();

        // Nodes
        world.add_node(0, (0.0, 0.0)); // A
        world.add_node(1, (10.0, 0.0)); // B
        world.add_node(2, (20.0, 0.0)); // C

        // Links
        // A -> B: Cheap
        world.add_link(0, 0, 1, 10, 1.0, 1.0);
        // B -> C: Cheap
        world.add_link(1, 1, 2, 10, 1.0, 1.0);
        // A -> C: Expensive Direct
        world.add_link(2, 0, 2, 10, 5.0, 1.0);

        // Run updates twice to propagate B->C route to A
        world.update_routes();
        world.update_routes();

        // Check A's route to C
        let node_a = world.nodes.get(&NodeId(0)).unwrap();
        let (next_hop, cost) = node_a.routing_table.get(&NodeId(2)).unwrap();

        // Should go via Link 0 (A->B)
        assert_eq!(next_hop.0, 0);
        assert!(*cost < 5.0); // 1.0 + 1.0 = 2.0

        // Now flood A->B
        let link_ab = world.links.get_mut(&LinkId(0)).unwrap();
        for i in 0..20 {
            link_ab.queue.push_back(PacketId(i));
        }
        link_ab.update_cost();

        // Cost should be high: 1.0 * (1 + (20/10)^2) = 1 * (1 + 4) = 5.0
        assert!(link_ab.current_cost >= 5.0);

        // Update routes
        world.update_routes();

        let node_a = world.nodes.get(&NodeId(0)).unwrap();
        let (next_hop, cost) = node_a.routing_table.get(&NodeId(2)).unwrap();

        // Should go via Link 2 (A->C) which has cost 5.0
        // If A->B cost is exactly 5.0, then path via B is 5.0 + 1.0 (Link 1) = 6.0.
        // So Direct (5.0) is cheaper.
        assert_eq!(next_hop.0, 2);
    }
}
