
use rand::Rng;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 64;

#[derive(Clone, Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<f32>,
    pub water: Vec<f32>,
    pub flux: Vec<[f32; 4]>, // L, R, T, B
    pub velocity: Vec<[f32; 2]>, // u, v
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut map = Map {
            width,
            height,
            terrain: vec![0.0; size],
            water: vec![0.0; size],
            flux: vec![[0.0; 4]; size],
            velocity: vec![[0.0; 2]; size],
        };
        map.generate_terrain();
        map
    }

    pub fn generate_terrain(&mut self) {
        let mut rng = rand::thread_rng();
        // Simple noise or just random bumps for now
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                // Create a bowl shape
                let dx = x as f32 - self.width as f32 / 2.0;
                let dy = y as f32 - self.height as f32 / 2.0;
                let dist = (dx * dx + dy * dy).sqrt();

                // Base height + random noise
                let base = (dist / 10.0).max(0.0);
                let noise = rng.gen_range(-1.0..1.0);

                self.terrain[idx] = (base + noise).max(0.0);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = Map::new(10, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.terrain.len(), 100);
        assert_eq!(map.water.len(), 100);
    }
}
