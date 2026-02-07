use macroquad::prelude::*;
use crate::landscape::ObjectiveFunction;
use crate::ant::{Ant, AntState};

const GRID_SIZE: usize = 100;
const WORLD_SIZE: f32 = 20.0; // -10 to 10
const CELL_SIZE: f32 = WORLD_SIZE / GRID_SIZE as f32;

pub struct World {
    pub func: Box<dyn ObjectiveFunction>,
    pub ants: Vec<Ant>,
    pub bridge_grid: Vec<f32>, // Max height of bridge ants in this cell
}

impl World {
    pub fn new(func: Box<dyn ObjectiveFunction>) -> Self {
        Self {
            func,
            ants: Vec::new(),
            bridge_grid: vec![-999.0; GRID_SIZE * GRID_SIZE],
        }
    }

    pub fn add_ants(&mut self, count: usize) {
        for _ in 0..count {
            let x = macroquad::rand::gen_range(-10.0, 10.0);
            let y = macroquad::rand::gen_range(-10.0, 10.0);
            self.ants.push(Ant::new(vec2(x, y)));
        }
    }

    // Helper used inside closure, duplicating logic slightly to avoid borrow issues
    // or we can make this static/pure function if we pass grid/func.

    pub fn update(&mut self) {
        // Update bridge grid based on current bridging ants
        // We reset it or keep it?
        // Since bridging ants are static and persistent, we can rebuild it to be safe (handle moved ants if any).
        // Or just update it incrementally. Rebuild is safer.
        self.bridge_grid.fill(-999.0);

        for ant in &self.ants {
            if ant.state == AntState::Bridging {
                let gx = ((ant.pos.x + 10.0) / CELL_SIZE) as i32;
                let gy = ((ant.pos.y + 10.0) / CELL_SIZE) as i32;
                if gx >= 0 && gx < GRID_SIZE as i32 && gy >= 0 && gy < GRID_SIZE as i32 {
                    let idx = (gy as usize) * GRID_SIZE + (gx as usize);
                    let h = self.func.value(ant.pos.x, ant.pos.y);
                    if h > self.bridge_grid[idx] {
                        self.bridge_grid[idx] = h;
                    }
                }
            }
        }

        let func = &self.func;
        let grid = &self.bridge_grid;

        let get_h = |pos: Vec2| -> f32 {
             let base = func.value(pos.x, pos.y);
             let gx = ((pos.x + 10.0) / CELL_SIZE) as i32;
             let gy = ((pos.y + 10.0) / CELL_SIZE) as i32;
             if gx >= 0 && gx < GRID_SIZE as i32 && gy >= 0 && gy < GRID_SIZE as i32 {
                 let bridge_h = grid[(gy as usize) * GRID_SIZE + (gx as usize)];
                 // If bridge_h is -999, base wins.
                 base.max(bridge_h)
             } else {
                 base
             }
        };

        let get_g = |pos: Vec2| -> Vec2 {
             let h = 0.1;
             let dx = (get_h(pos + vec2(h, 0.0)) - get_h(pos - vec2(h, 0.0))) / (2.0 * h);
             let dy = (get_h(pos + vec2(0.0, h)) - get_h(pos - vec2(0.0, h))) / (2.0 * h);
             Vec2::new(dx, dy)
        };

        for ant in &mut self.ants {
            ant.update(&get_h, &get_g, 0.016);
        }
    }
}
