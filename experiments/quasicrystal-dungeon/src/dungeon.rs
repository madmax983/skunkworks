use crate::math::Quasicrystal;
use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoomType {
    Empty,
    Start,
    Goal,
    Treasure,
    Enemy,
    Trap,
    Boss,
}

pub struct Dungeon {
    pub lattice: Quasicrystal,
    pub player_idx: usize,
    pub visited: HashSet<usize>,
    #[allow(dead_code)]
    pub goal_idx: usize,
    pub room_types: HashMap<usize, RoomType>,
}

impl Dungeon {
    pub fn new(lattice: Quasicrystal) -> Self {
        // Pick a start index. If lattice is empty, handle gracefully (though it shouldn't be).
        if lattice.atoms.is_empty() {
            return Dungeon {
                lattice,
                player_idx: 0,
                visited: HashSet::new(),
                goal_idx: 0,
                room_types: HashMap::new(),
            };
        }

        // Try to start near center (index of closest atom to origin).
        // Since atoms are not sorted by distance, we scan.
        let mut start_idx = 0;
        let mut min_dist = f32::MAX;

        for (i, p) in lattice.atoms.iter().enumerate() {
            let d = p.x * p.x + p.y * p.y + p.z * p.z;
            if d < min_dist {
                min_dist = d;
                start_idx = i;
            }
        }

        let mut visited = HashSet::new();
        visited.insert(start_idx);

        // BFS to find furthest node for Goal
        let goal_idx = Self::find_furthest_node(&lattice, start_idx);

        let room_types = Self::populate_dungeon(&lattice, start_idx, goal_idx);

        Dungeon {
            lattice,
            player_idx: start_idx,
            visited,
            goal_idx,
            room_types,
        }
    }

    fn populate_dungeon(
        lattice: &Quasicrystal,
        start_idx: usize,
        goal_idx: usize,
    ) -> HashMap<usize, RoomType> {
        let mut room_types = HashMap::new();
        let mut rng = rand::thread_rng();

        room_types.insert(start_idx, RoomType::Start);
        room_types.insert(goal_idx, RoomType::Goal);

        for (i, _) in lattice.atoms.iter().enumerate() {
            if i == start_idx || i == goal_idx {
                continue;
            }

            let neighbors = lattice.adj[i].len();
            let room = if neighbors >= 10 {
                // High connectivity: Hub
                let roll = rng.gen_range(0..100);
                if roll < 20 {
                    RoomType::Treasure
                } else if roll < 25 {
                    RoomType::Boss
                } else {
                    RoomType::Empty
                }
            } else if neighbors <= 5 {
                // Low connectivity: Corridor / Dead end
                let roll = rng.gen_range(0..100);
                if roll < 10 {
                    RoomType::Enemy
                } else if roll < 20 {
                    RoomType::Trap
                } else {
                    RoomType::Empty
                }
            } else {
                // Average
                let roll = rng.gen_range(0..100);
                if roll < 5 {
                    RoomType::Enemy
                } else {
                    RoomType::Empty
                }
            };

            if room != RoomType::Empty {
                room_types.insert(i, room);
            }
        }
        room_types
    }

    fn find_furthest_node(lattice: &Quasicrystal, start_idx: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((start_idx, 0));
        let mut visited = HashSet::new();
        visited.insert(start_idx);

        let mut furthest_idx = start_idx;
        let mut max_dist = 0;

        while let Some((curr, dist)) = queue.pop_front() {
            if dist > max_dist {
                max_dist = dist;
                furthest_idx = curr;
            }

            if curr >= lattice.adj.len() {
                continue;
            } // Safety check

            for &neighbor in &lattice.adj[curr] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
        furthest_idx
    }

    pub fn move_player(&mut self, target_idx: usize) -> bool {
        if self.player_idx >= self.lattice.adj.len() {
            return false;
        }

        if self.lattice.adj[self.player_idx].contains(&target_idx) {
            self.player_idx = target_idx;
            self.visited.insert(target_idx);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::generate_icosahedral_lattice;

    #[test]
    fn test_dungeon_initialization() {
        let qc = generate_icosahedral_lattice(2);
        let dungeon = Dungeon::new(qc);

        assert!(dungeon.visited.contains(&dungeon.player_idx));
        assert_eq!(
            dungeon.room_types.get(&dungeon.player_idx),
            Some(&RoomType::Start)
        );
        assert_eq!(
            dungeon.room_types.get(&dungeon.goal_idx),
            Some(&RoomType::Goal)
        );
        assert_ne!(dungeon.player_idx, dungeon.goal_idx); // Should be far apart
    }

    #[test]
    fn test_player_movement() {
        let qc = generate_icosahedral_lattice(2);
        let mut dungeon = Dungeon::new(qc);

        let start = dungeon.player_idx;
        let neighbors = &dungeon.lattice.adj[start];
        assert!(!neighbors.is_empty());

        let target = neighbors[0];
        assert!(dungeon.move_player(target));
        assert_eq!(dungeon.player_idx, target);
        assert!(dungeon.visited.contains(&target));

        // Try invalid move
        // Find a node that is not a neighbor
        // Simple heuristic: pick node with index (target+2) % len
        // Unless graph is fully connected (it's not), this likely works.
        // Better: iterate all nodes, pick one not in adj[target].
        let mut invalid_target = 0;
        for i in 0..dungeon.lattice.atoms.len() {
            if !dungeon.lattice.adj[target].contains(&i) && i != target {
                invalid_target = i;
                break;
            }
        }

        assert!(!dungeon.move_player(invalid_target));
        assert_eq!(dungeon.player_idx, target);
    }
}
