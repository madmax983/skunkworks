use crate::graph::CrateGraph;
use crate::layout::Vec3;
use noise::{NoiseFn, Perlin};
use petgraph::graph::NodeIndex;
use rand::Rng;
use std::collections::HashMap;

pub const GRID_SIZE: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Voxel {
    Rock,
    Air,
    Crack,
}

pub struct VoxelGrid {
    pub voxels: Vec<Voxel>,
    pub void_count: usize,
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl VoxelGrid {
    pub fn new() -> Self {
        let perlin = Perlin::new(1);
        let mut voxels = vec![Voxel::Rock; GRID_SIZE * GRID_SIZE * GRID_SIZE];
        let void_count = 0;

        // Initialize with faults
        for z in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1]);
                    let idx = x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE;
                    // Perlin noise typically returns -1.0 to 1.0
                    if val > 0.2 {
                        voxels[idx] = Voxel::Crack;
                    } else {
                        voxels[idx] = Voxel::Rock;
                    }
                }
            }
        }

        Self {
            voxels,
            void_count,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Voxel {
        if x >= GRID_SIZE || y >= GRID_SIZE || z >= GRID_SIZE {
            return Voxel::Rock;
        }
        self.voxels[x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: Voxel) {
        if x >= GRID_SIZE || y >= GRID_SIZE || z >= GRID_SIZE {
            return;
        }
        let idx = x + y * GRID_SIZE + z * GRID_SIZE * GRID_SIZE;
        if self.voxels[idx] != Voxel::Air && voxel == Voxel::Air {
            self.void_count += 1;
        }
        self.voxels[idx] = voxel;
    }

    pub fn erode(&mut self, graph: &CrateGraph, positions: &HashMap<NodeIndex, Vec3>) {
        let size_f = GRID_SIZE as f32;

        for edge in graph.edge_indices() {
            if let Some((u, v)) = graph.edge_endpoints(edge) {
                // Check if nodes exist in layout (they should)
                if let (Some(start), Some(end)) = (positions.get(&u), positions.get(&v)) {
                    let start_grid = (
                        (start.x * size_f) as isize,
                        (start.y * size_f) as isize,
                        (start.z * size_f) as isize,
                    );
                    let end_grid = (
                        (end.x * size_f) as isize,
                        (end.y * size_f) as isize,
                        (end.z * size_f) as isize,
                    );

                    self.carve_line(start_grid, end_grid);
                }
            }
        }
    }

    fn carve_line(&mut self, start: (isize, isize, isize), end: (isize, isize, isize)) {
        let (x1, y1, z1) = start;
        let (x2, y2, z2) = end;

        let dx = (x2 - x1) as f32;
        let dy = (y2 - y1) as f32;
        let dz = (z2 - z1) as f32;

        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let steps = dist as usize;

        if steps == 0 {
            return;
        }

        let step_x = dx / steps as f32;
        let step_y = dy / steps as f32;
        let step_z = dz / steps as f32;

        let mut curr_x = x1 as f32;
        let mut curr_y = y1 as f32;
        let mut curr_z = z1 as f32;

        let mut rng = rand::thread_rng();

        for _ in 0..=steps {
            // Wiggle
            let wx = rng.gen_range(-0.5..0.5);
            let wy = rng.gen_range(-0.5..0.5);
            let wz = rng.gen_range(-0.5..0.5);

            let tx = (curr_x + wx).round() as isize;
            let ty = (curr_y + wy).round() as isize;
            let tz = (curr_z + wz).round() as isize;

            if tx >= 0 && ty >= 0 && tz >= 0 {
                self.set(tx as usize, ty as usize, tz as usize, Voxel::Air);
            }

            curr_x += step_x;
            curr_y += step_y;
            curr_z += step_z;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use petgraph::Graph;

    #[test]
    fn test_erosion() {
        let mut graph = Graph::new();
        let n1 = graph.add_node("A".to_string());
        let n2 = graph.add_node("B".to_string());
        graph.add_edge(n1, n2, ());

        let mut positions = HashMap::new();
        positions.insert(n1, Vec3::new(0.1, 0.5, 0.5));
        positions.insert(n2, Vec3::new(0.9, 0.5, 0.5));

        let mut grid = VoxelGrid::new();
        let initial_voids = grid.void_count;

        grid.erode(&graph, &positions);

        assert!(grid.void_count > initial_voids);
    }
}
