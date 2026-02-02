use std::collections::HashMap;
use std::cell::{RefCell, Ref};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    // Maybe Add Special Types later
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub visited: bool,
    pub color_seed: u64, // For rendering variation
}

pub type Path = Vec<usize>;

pub struct Dungeon {
    tiles: RefCell<HashMap<Path, Tile>>,
    seed: u64,
}

impl Dungeon {
    pub fn new(seed: u64) -> Self {
        let mut tiles = HashMap::new();
        // Ensure root exists and is floor
        tiles.insert(vec![], Tile {
            tile_type: TileType::Floor,
            visited: true,
            color_seed: 0,
        });

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
        // Return Ref. We need to release the borrow_mut first.
        drop(tiles);

        // This pattern (borrow_mut then borrow) works because we dropped the mutable borrow.
        // But to return a Ref that maps to a value inside, we need `Ref::map`.
        let tiles = self.tiles.borrow();
        Ref::map(tiles, |t| t.get(path).unwrap())
    }

    pub fn mark_visited(&self, path: &Path) {
        let mut tiles = self.tiles.borrow_mut();
        if let Some(tile) = tiles.get_mut(path) {
            tile.visited = true;
        }
    }

    fn generate_tile(&self, path: &Path) -> Tile {
        // Hash the path + seed to get a deterministic RNG
        let mut hasher = DefaultHasher::new();
        self.seed.hash(&mut hasher);
        path.hash(&mut hasher);
        let hash = hasher.finish();
        let mut rng = StdRng::seed_from_u64(hash);

        // 20% chance of wall, but path cannot be wall if we are just checking it
        // Wait, if we move into a wall, we block.
        // But for generation, we decide what it is.
        let is_wall = rng.gen_bool(0.2);

        // Root is always floor (handled in new)

        Tile {
            tile_type: if is_wall { TileType::Wall } else { TileType::Floor },
            visited: false,
            color_seed: rng.gen(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize() {
        let path = vec![0, 1];
        // Move opposite to 1 (which is 3)
        let new_path = Dungeon::canonicalize_step(path.clone(), 3);
        assert_eq!(new_path, vec![0]);

        // Move arbitrary (2) which is opposite to 0? No, 2 is opposite to 0.
        // path is [0, 1]. Last is 1. 2 is not opposite to 1.
        let path2 = vec![0, 1];
        let new_path2 = Dungeon::canonicalize_step(path2, 2);
        assert_eq!(new_path2, vec![0, 1, 2]);
    }
}
