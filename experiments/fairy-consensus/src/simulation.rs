use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeState {
    Follower,
    Candidate,
    Leader,
    Committed,
}

#[derive(Debug)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub state: NodeState,
    pub timer: u32, // Countdown to candidacy
}

impl Node {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            state: NodeState::Follower,
            timer: 0, // Initialized by World or randomly
        }
    }

    pub fn reset_timer<R: Rng>(&mut self, rng: &mut R) {
        self.timer = rng.gen_range(100..300);
    }
}

#[derive(Debug)]
pub struct Ring {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub term: u64,
}

impl Ring {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, radius: 0.0, term: 0 }
    }

    pub fn expand(&mut self) {
        self.radius += 0.5;
    }

    pub fn interact(&self, node: &mut Node) -> bool {
        let dx = self.x - node.x;
        let dy = self.y - node.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // Check if the ring's wavefront hits the node
        if (dist - self.radius).abs() < 1.0 {
            if node.state == NodeState::Follower || node.state == NodeState::Candidate {
                // If hit by a ring, we accept the leader (simplification)
                // In full Raft, check terms. Here, just visualize propagation.
                node.state = NodeState::Committed;
                // Reset timer to prevent becoming candidate immediately
                // node.timer = 100;
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct World {
    pub nodes: Vec<Node>,
    pub rings: Vec<Ring>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            nodes: Vec::new(),
            rings: Vec::new(),
            width,
            height,
        }
    }

    pub fn update<R: Rng>(&mut self, rng: &mut R) {
        // Expand rings
        for ring in &mut self.rings {
            ring.expand();
        }

        // Interactions
        for node in &mut self.nodes {
            let mut hit = false;
            for ring in &self.rings {
                if ring.interact(node) {
                    hit = true;
                }
            }

            // Logic:
            if hit {
                node.reset_timer(rng); // Reset election timer on heartbeat
            }

            match node.state {
                NodeState::Follower | NodeState::Candidate => {
                    if node.timer == 0 {
                        // Timeout! Become Candidate/Leader and broadcast
                        node.state = NodeState::Leader;
                        node.reset_timer(rng);
                    } else {
                        node.timer -= 1;
                    }
                }
                NodeState::Leader => {
                    // Leaders send heartbeats periodically
                    if node.timer == 0 {
                        // We need to spawn a ring.
                        // Since we can't mutate self.rings while iterating self.nodes,
                        // we'll handle spawning in a second pass or collect events.
                        // For now, just reset timer.
                         node.reset_timer(rng);
                    } else {
                        node.timer -= 1;
                    }
                }
                NodeState::Committed => {
                     // Committed nodes might revert to Follower eventually or spread logic
                     if rng.gen_bool(0.001) {
                         node.state = NodeState::Follower;
                     }
                }
            }
        }

        // Spawn rings phase
        let mut new_rings = Vec::new();

        // Let's refine the spawn logic:
        // We need to know WHICH nodes want to spawn rings.
        // We can't mutate `self.rings` inside the `nodes` loop.
        // We can iterate indices?

        for i in 0..self.nodes.len() {
             let node = &mut self.nodes[i];
             if node.state == NodeState::Leader && node.timer % 50 == 0 {
                 new_rings.push(Ring::new(node.x, node.y));
             }
        }
        self.rings.append(&mut new_rings);

        // Cleanup rings
        self.rings.retain(|r| r.radius < self.width.max(self.height) * 1.5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_expansion() {
        let mut ring = Ring::new(10.0, 10.0);
        let initial_radius = ring.radius;
        ring.expand();
        assert!(ring.radius > initial_radius, "Ring should expand");
    }

    #[test]
    fn test_node_voting() {
        let mut node = Node::new(10.0, 15.0);
        let mut ring = Ring::new(10.0, 10.0);

        ring.radius = 5.0;

        let voted = ring.interact(&mut node);
        assert!(voted, "Node should vote when touched by ring");
        assert_eq!(node.state, NodeState::Committed, "Node state should change to Committed");
    }

    #[test]
    fn test_world_update() {
        let mut world = World::new(100.0, 100.0);
        let mut node = Node::new(10.0, 15.0);
        node.timer = 1000; // Prevent spontaneous leadership
        world.nodes.push(node);
        world.rings.push(Ring::new(10.0, 10.0));

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            world.update(&mut rng);
        }

        assert_eq!(world.nodes[0].state, NodeState::Committed);
    }
}
