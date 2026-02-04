use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cell::{Ref, RefCell};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub visited: bool,
    pub color_seed: u64,
    pub has_food: bool,
    pub pheromone_food: f64,
}

pub type Path = Vec<usize>;

pub struct Dungeon {
    tiles: RefCell<HashMap<Path, Tile>>,
    seed: u64,
}

impl Dungeon {
    pub fn new(seed: u64) -> Self {
        let mut tiles = HashMap::new();
        // Ensure root exists and is floor (Nest)
        tiles.insert(
            vec![],
            Tile {
                tile_type: TileType::Floor,
                visited: true,
                color_seed: 0,
                has_food: false, // Nest has no food to pick
                pheromone_food: 0.0,
            },
        );

        Self {
            tiles: RefCell::new(tiles),
            seed,
        }
    }

    /// Canonicalize path: In a tree structure, stepping back cancels the previous step.
    /// Opposites: 0<->2, 1<->3 (Right<->Left, Up<->Down)
    pub fn canonicalize_step(mut path: Path, step: usize) -> Path {
        if let Some(&last) = path.last() {
            let opposite = (step + 2) % 4;
            if last == opposite {
                path.pop();
                return path;
            }
        }
        path.push(step);
        path
    }

    pub fn get_tile(&self, path: &Path) -> Ref<'_, Tile> {
        let mut tiles = self.tiles.borrow_mut();
        if !tiles.contains_key(path) {
            let tile = self.generate_tile(path);
            tiles.insert(path.clone(), tile);
        }
        drop(tiles);

        let tiles = self.tiles.borrow();
        Ref::map(tiles, |t| t.get(path).unwrap())
    }

    pub fn mark_visited(&self, path: &Path) {
        let mut tiles = self.tiles.borrow_mut();
        if let Some(tile) = tiles.get_mut(path) {
            tile.visited = true;
        }
    }

    pub fn deposit_pheromone(&self, path: &Path, amount: f64) {
        // Ensure tile exists
        {
            let mut tiles = self.tiles.borrow_mut();
            if !tiles.contains_key(path) {
                let tile = self.generate_tile(path);
                tiles.insert(path.clone(), tile);
            }
        }

        let mut tiles = self.tiles.borrow_mut();
        if let Some(tile) = tiles.get_mut(path) {
            tile.pheromone_food = (tile.pheromone_food + amount).min(100.0);
        }
    }

    pub fn decay_pheromones(&self) {
        let mut tiles = self.tiles.borrow_mut();
        for tile in tiles.values_mut() {
            tile.pheromone_food *= 0.98; // 2% decay per tick
            if tile.pheromone_food < 0.05 {
                tile.pheromone_food = 0.0;
            }
        }
    }

    pub fn try_pick_food(&self, path: &Path) -> bool {
        let mut tiles = self.tiles.borrow_mut();
        if let Some(tile) = tiles.get_mut(path) {
            if tile.has_food {
                tile.has_food = false;
                return true;
            }
        }
        false
    }

    // Helper to peek neighbor pheromones without generating them if possible?
    // Actually we need to generate them to see if they are walls or have food/pheromone.
    // So agents exploring will generate the map. This is fine.

    fn generate_tile(&self, path: &Path) -> Tile {
        let mut hasher = DefaultHasher::new();
        self.seed.hash(&mut hasher);
        path.hash(&mut hasher);
        let hash = hasher.finish();
        let mut rng = StdRng::seed_from_u64(hash);

        // 10% chance of wall
        let is_wall = rng.gen_bool(0.1);

        // 5% chance of food (if not wall)
        let has_food = !is_wall && rng.gen_bool(0.05);

        Tile {
            tile_type: if is_wall {
                TileType::Wall
            } else {
                TileType::Floor
            },
            visited: false,
            color_seed: rng.gen(),
            has_food,
            pheromone_food: 0.0,
        }
    }
}
