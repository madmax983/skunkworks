use macroquad::prelude::*;

pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 30;
pub const TILE_SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Food,
}

#[derive(Clone)]
pub struct Portal {
    pub id: usize,
    pub entry_pos: (usize, usize),
    pub exit_pos: (usize, usize),
    pub color_entry: Color,
    pub color_exit: Color,
    pub energy: i32,
    pub owner_id: u64,
}

pub struct World {
    pub grid: [[Tile; GRID_HEIGHT]; GRID_WIDTH],
    pub portals: Vec<Portal>,
    pub food_count: usize,
}

impl World {
    pub fn new() -> Self {
        let mut grid = [[Tile::Empty; GRID_HEIGHT]; GRID_WIDTH];

        // Generate Walls (Maze-like or Perlin?)
        // Simple random walls for now
        for (x, col) in grid.iter_mut().enumerate().take(GRID_WIDTH) {
            for (y, tile) in col.iter_mut().enumerate().take(GRID_HEIGHT) {
                if x == 0
                    || x == GRID_WIDTH - 1
                    || y == 0
                    || y == GRID_HEIGHT - 1
                    || ::rand::random::<f32>() < 0.1
                {
                    *tile = Tile::Wall;
                }
            }
        }

        let mut world = Self {
            grid,
            portals: Vec::new(),
            food_count: 0,
        };

        world.spawn_initial_food(50);
        world
    }

    pub fn spawn_initial_food(&mut self, amount: usize) {
        for _ in 0..amount {
            self.spawn_food();
        }
    }

    pub fn spawn_food(&mut self) {
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;

        loop {
            let x = rng.gen_range(1..GRID_WIDTH - 1);
            let y = rng.gen_range(1..GRID_HEIGHT - 1);
            if self.grid[x][y] == Tile::Empty {
                self.grid[x][y] = Tile::Food;
                self.food_count += 1;
                break;
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        if x >= GRID_WIDTH || y >= GRID_HEIGHT {
            return Tile::Wall;
        }
        self.grid[x][y]
    }

    pub fn consume_food(&mut self, x: usize, y: usize) -> bool {
        if x < GRID_WIDTH && y < GRID_HEIGHT && self.grid[x][y] == Tile::Food {
            self.grid[x][y] = Tile::Empty;
            self.food_count -= 1;
            self.spawn_food(); // Replenish
            return true;
        }
        false
    }

    pub fn add_portal(&mut self, entry: (usize, usize), exit: (usize, usize), owner: u64) {
        let id = self.portals.len();
        self.portals.push(Portal {
            id,
            entry_pos: entry,
            exit_pos: exit,
            color_entry: BLUE,
            color_exit: ORANGE,
            energy: 500, // Decays over time
            owner_id: owner,
        });
    }

    pub fn check_portal(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        for portal in &self.portals {
            if portal.entry_pos == (x, y) {
                return Some(portal.exit_pos);
            }
            if portal.exit_pos == (x, y) {
                return Some(portal.entry_pos);
            }
        }
        None
    }

    pub fn update(&mut self) {
        // Decay portals
        self.portals.retain_mut(|p| {
            p.energy -= 1;
            p.energy > 0
        });
    }

    pub fn render(&self) {
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let pos_x = x as f32 * TILE_SIZE;
                let pos_y = y as f32 * TILE_SIZE;

                match self.grid[x][y] {
                    Tile::Wall => draw_rectangle(pos_x, pos_y, TILE_SIZE, TILE_SIZE, DARKGRAY),
                    Tile::Food => draw_rectangle(
                        pos_x + 5.0,
                        pos_y + 5.0,
                        TILE_SIZE - 10.0,
                        TILE_SIZE - 10.0,
                        GREEN,
                    ),
                    _ => {}
                }

                // Draw Grid lines
                draw_rectangle_lines(
                    pos_x,
                    pos_y,
                    TILE_SIZE,
                    TILE_SIZE,
                    1.0,
                    Color::new(0.2, 0.2, 0.2, 0.1),
                );
            }
        }

        // Draw Portals
        for portal in &self.portals {
            let (ex, ey) = portal.entry_pos;
            let (xx, xy) = portal.exit_pos;

            draw_circle(
                ex as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                ey as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                TILE_SIZE / 2.0,
                portal.color_entry,
            );
            draw_circle(
                xx as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                xy as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                TILE_SIZE / 2.0,
                portal.color_exit,
            );

            // Draw link line
            draw_line(
                ex as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                ey as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                xx as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                xy as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                2.0,
                Color::new(0.5, 0.0, 0.5, 0.5),
            );
        }
    }
}
