use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoxelType {
    Rock,     // Base/Stable code
    Sediment, // New/Transient code
    Fossil,   // Deleted/Legacy code (buried)
    Bone,     // Bugs/Fixes (hard deposits)
}

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub color: Color,
}

impl Voxel {
    pub fn new(voxel_type: VoxelType, color: Color) -> Self {
        Self { voxel_type, color }
    }
}

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    // A 2D grid where each cell is a vertical stack of voxels.
    // grid[y * width + x] is a Vec<Voxel> representing the column at (x, y).
    // Index 0 is the bottom, last index is the top.
    pub grid: Vec<Vec<Voxel>>,
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            grid.push(Vec::new());
        }
        Self { width, height, grid }
    }

    pub fn get_stack(&self, x: usize, y: usize) -> Option<&Vec<Voxel>> {
        if x < self.width && y < self.height {
            Some(&self.grid[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_stack_mut(&mut self, x: usize, y: usize) -> Option<&mut Vec<Voxel>> {
        if x < self.width && y < self.height {
            Some(&mut self.grid[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_height(&self, x: usize, y: usize) -> usize {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x].len()
        } else {
            0
        }
    }

    pub fn push_voxel(&mut self, x: usize, y: usize, voxel: Voxel) {
        if let Some(stack) = self.get_stack_mut(x, y) {
            stack.push(voxel);
        }
    }

    pub fn pop_voxel(&mut self, x: usize, y: usize) -> Option<Voxel> {
        if let Some(stack) = self.get_stack_mut(x, y) {
            stack.pop()
        } else {
            None
        }
    }

    pub fn peek_voxel(&self, x: usize, y: usize) -> Option<&Voxel> {
         if let Some(stack) = self.get_stack(x, y) {
            stack.last()
        } else {
            None
        }
    }
}
