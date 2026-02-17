use crate::math::Quasicrystal;
use crate::sim::World;
use std::sync::Arc;

pub struct Dungeon {
    pub world: World,
    pub player_idx: usize, // Cursor / Focus
}

impl Dungeon {
    pub fn new(lattice: Quasicrystal) -> Self {
        let lattice = Arc::new(lattice);
        let world = World::new(lattice.clone());
        let player_idx = world.center_node;

        Dungeon {
            world,
            player_idx,
        }
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    // Helper to move player cursor
    pub fn move_player(&mut self, target_idx: usize) -> bool {
        if self.world.qc.adj[self.player_idx].contains(&target_idx) {
            self.player_idx = target_idx;
            return true;
        }
        false
    }
}
