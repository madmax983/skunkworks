use macroquad::prelude::*;
use std::collections::HashMap;
use crate::nile::EgyptianFraction;
use num_rational::Ratio;

#[derive(Debug, Clone, PartialEq)]
pub enum ColonyState {
    Spore,
    Growing,
    Mature,
    Decaying,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub colony_id: usize,
    pub pos: Vec2,
    pub size: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Colony {
    pub id: usize,
    pub denominator: u64, // The 'd' in 1/d
    pub target_area: f32,
    pub current_area: f32,
    pub nodes: Vec<usize>,
    pub center_of_mass: Vec2,
    pub state: ColonyState,
    pub age: f32,
    pub lifespan: f32,
    pub color: Color,
}

pub struct Floodplain {
    pub nodes: HashMap<usize, Node>,
    pub colonies: HashMap<usize, Colony>,
    pub next_node_id: usize,
    pub next_colony_id: usize,
    pub bounds: Rect,
}

impl Floodplain {
    pub fn new(bounds: Rect) -> Self {
        Self {
            nodes: HashMap::new(),
            colonies: HashMap::new(),
            next_node_id: 0,
            next_colony_id: 0,
            bounds,
        }
    }

    pub fn add_colony(&mut self, denominator: u64, pos: Vec2, total_area_reference: f32) {
        let id = self.next_colony_id;
        self.next_colony_id += 1;

        let target_area = total_area_reference / (denominator as f32);

        // Color based on denominator (Egyptian aesthetic)
        // 1/2 -> Gold
        // 1/small -> Green/Lapis
        // 1/large -> Sand
        let color = match denominator {
            2 => GOLD,
            3 => Color::new(0.0, 0.8, 0.2, 1.0), // Emerald
            4..=10 => Color::new(0.0, 0.4, 0.8, 1.0), // Lapis Lazuli
            _ => Color::new(0.8, 0.7, 0.5, 1.0), // Sand
        };

        let colony = Colony {
            id,
            denominator,
            target_area,
            current_area: 0.0,
            nodes: Vec::new(),
            center_of_mass: pos,
            state: ColonyState::Spore,
            age: 0.0,
            lifespan: 10.0 + (denominator as f32), // Smaller fractions live longer? Or shorter?
            color,
        };

        self.colonies.insert(id, colony);

        // Spawn initial spore node
        self.spawn_node(id, pos, 5.0);
    }

    fn spawn_node(&mut self, colony_id: usize, pos: Vec2, size: f32) -> bool {
        // Check bounds
        if !self.bounds.contains(pos) {
            return false;
        }

        // Collision Check (Naive)
        // Don't spawn if overlapping significantly with ANY existing node
        for other in self.nodes.values() {
            let dist = pos.distance(other.pos);
            let min_dist = (size + other.size) * 0.8; // Allow slight overlap for organic look
            if dist < min_dist {
                return false;
            }
        }

        let id = self.next_node_id;
        self.next_node_id += 1;

        let node = Node {
            id,
            colony_id,
            pos,
            size,
            color: self.colonies[&colony_id].color,
        };

        self.nodes.insert(id, node);

        if let Some(colony) = self.colonies.get_mut(&colony_id) {
            colony.nodes.push(id);
            colony.current_area += std::f32::consts::PI * size * size;
        }

        true
    }

    pub fn update(&mut self, dt: f32) {
        let mut to_remove_colonies = Vec::new();

        // 1. Logic per colony
        // We need to collect mutations to avoid borrow checker
        // Or iterate keys
        let colony_ids: Vec<usize> = self.colonies.keys().cloned().collect();

        for id in colony_ids {
            // Re-borrow colony
            let mut colony_status = ColonyState::Spore; // Placeholder
            let mut should_grow = false;
            let mut grow_attempts = 0;
            let mut parent_pos = Vec2::ZERO;

            if let Some(colony) = self.colonies.get_mut(&id) {
                colony.age += dt;

                // State transitions
                match colony.state {
                    ColonyState::Spore => {
                         if colony.current_area < colony.target_area {
                             colony.state = ColonyState::Growing;
                         }
                    },
                    ColonyState::Growing => {
                        if colony.current_area >= colony.target_area {
                            colony.state = ColonyState::Mature;
                        } else if colony.age > colony.lifespan * 2.0 {
                            // Stunted growth, eventually die
                             colony.state = ColonyState::Decaying;
                        }
                    },
                    ColonyState::Mature => {
                        if colony.age > colony.lifespan {
                            colony.state = ColonyState::Decaying;
                        }
                    },
                    ColonyState::Decaying => {
                         if colony.nodes.is_empty() {
                             to_remove_colonies.push(id);
                         }
                    }
                }

                colony_status = colony.state.clone();

                // Prepare for growth if needed
                if colony_status == ColonyState::Growing {
                    should_grow = true;
                    if !colony.nodes.is_empty() {
                         let idx = rand::gen_range(0, colony.nodes.len());
                         let node_id = colony.nodes[idx];
                         if let Some(n) = self.nodes.get(&node_id) {
                             parent_pos = n.pos;
                         }
                    } else {
                        parent_pos = colony.center_of_mass;
                    }
                }
            }

            // Execute Growth
            if should_grow {
                // Try to spawn a child nearby
                let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
                let dist = rand::gen_range(10.0, 20.0);
                let new_pos = parent_pos + vec2(angle.cos() * dist, angle.sin() * dist);
                let size = rand::gen_range(3.0, 8.0);

                // Add jitter to color
                self.spawn_node(id, new_pos, size);
            }

            // Execute Decay (Shrink nodes)
             if colony_status == ColonyState::Decaying {
                 if let Some(colony) = self.colonies.get_mut(&id) {
                     // Change color to brown
                     colony.color = BROWN;

                     // Collect nodes to remove
                     let mut dead_nodes = Vec::new();
                     colony.nodes.retain(|&nid| {
                         let keep = if let Some(n) = self.nodes.get_mut(&nid) {
                             n.size -= dt * 2.0;
                             n.color = BROWN;
                             if n.size <= 0.5 {
                                 dead_nodes.push(nid);
                                 false
                             } else {
                                 true
                             }
                         } else {
                             false
                         };
                         keep
                     });

                     for nid in dead_nodes {
                         self.nodes.remove(&nid);
                     }
                 }
             }
        }

        // Cleanup empty colonies
        for id in to_remove_colonies {
            self.colonies.remove(&id);
        }
    }

    pub fn draw(&self) {
        // Draw links first?
        // Naive: just draw circles.

        for node in self.nodes.values() {
            draw_circle(node.pos.x, node.pos.y, node.size, node.color);
        }

        // Labels for colonies (1/d)
        for colony in self.colonies.values() {
            if !colony.nodes.is_empty() && colony.state != ColonyState::Decaying {
                // Approximate center
                 let center = if let Some(first) = colony.nodes.first().and_then(|id| self.nodes.get(id)) {
                     first.pos
                 } else {
                     colony.center_of_mass
                 };

                 draw_text(
                     &format!("1/{}", colony.denominator),
                     center.x - 10.0,
                     center.y,
                     20.0,
                     WHITE
                 );
            }
        }
    }
}
