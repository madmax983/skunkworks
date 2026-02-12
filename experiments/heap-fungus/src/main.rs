use rand::prelude::*;
use std::collections::{HashSet, VecDeque};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::{canvas::*, *}};
use std::io::{self};
use std::time::{Duration, Instant};

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 60;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FungalType {
    Marker,  // Symbiote (Blue/Cyan) - The GC Trace
    Sweeper, // Decomposer (Red/Orange) - The Garbage Collector
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct FungalCell {
    pub fungus_type: FungalType,
    pub age: u32,
    pub energy: f32, // 0.0 to 1.0. Markers decay if not fed. Sweepers decay if not eating.
}

impl Default for FungalCell {
    fn default() -> Self {
        Self {
            fungus_type: FungalType::None,
            age: 0,
            energy: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Object {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub size: usize, // Radius or block count
    pub references: Vec<usize>, // IDs of referenced objects
    pub active: bool, // If false, it's been eaten
    pub marked: bool, // Logical mark state (for debugging/visuals)
}

pub struct World {
    pub grid: Vec<FungalCell>, // Flattened grid
    pub width: usize,
    pub height: usize,
    pub objects: Vec<Object>,
    pub roots: Vec<usize>, // IDs of root objects
    pub tick: u64,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![FungalCell::default(); width * height],
            width,
            height,
            objects: Vec::new(),
            roots: Vec::new(),
            tick: 0,
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (y.clamp(0, self.height - 1)) * self.width + (x.clamp(0, self.width - 1))
    }

    pub fn get_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    // --- MUTATOR LOGIC ---

    pub fn allocate(&mut self) {
        let mut rng = rand::thread_rng();
        // Try to find a free spot
        for _ in 0..10 {
            let x = rng.gen_range(2..self.width-2);
            let y = rng.gen_range(2..self.height-2);

            let idx = self.get_index(x, y);
            if self.grid[idx].fungus_type == FungalType::None {
                let id = self.objects.len();
                let obj = Object {
                    id,
                    x,
                    y,
                    size: rng.gen_range(1..3),
                    references: Vec::new(),
                    active: true,
                    marked: false,
                };
                self.objects.push(obj);

                // 20% chance to be a root immediately
                if rng.gen_bool(0.2) {
                    self.roots.push(id);
                }
                break;
            }
        }
    }

    pub fn mutate_references(&mut self) {
        let mut rng = rand::thread_rng();
        let count = self.objects.len();
        if count < 2 { return; }

        // Create a link
        if rng.gen_bool(0.3) {
            let from = rng.gen_range(0..count);
            let to = rng.gen_range(0..count);
            if from != to
                && self.objects[from].active
                && self.objects[to].active
                && !self.objects[from].references.contains(&to)
            {
                self.objects[from].references.push(to);
            }
        }

        // Drop a link (Create Garbage)
        if rng.gen_bool(0.1) {
            let from = rng.gen_range(0..count);
            if self.objects[from].active && !self.objects[from].references.is_empty() {
                let r_idx = rng.gen_range(0..self.objects[from].references.len());
                self.objects[from].references.remove(r_idx);
            }
        }

        // Drop a root
        if rng.gen_bool(0.05) && !self.roots.is_empty() {
            let r_idx = rng.gen_range(0..self.roots.len());
            self.roots.remove(r_idx);
        }
    }

    // --- FUNGAL LOGIC ---

    pub fn update(&mut self) {
        self.tick += 1;
        let mut rng = rand::thread_rng();
        let width = self.width;
        let height = self.height;
        let get_idx = |x: usize, y: usize| -> usize {
            (y.clamp(0, height - 1)) * width + (x.clamp(0, width - 1))
        };

        // 1. Mutator runs occasionally
        if rng.gen_bool(0.1) { self.allocate(); }
        if rng.gen_bool(0.1) { self.mutate_references(); }

        // 2. Decay all cells
        for cell in &mut self.grid {
            if cell.energy > 0.0 {
                cell.energy -= 0.02; // Global decay
            }
            if cell.energy <= 0.0 {
                cell.fungus_type = FungalType::None;
                cell.energy = 0.0;
            }
        }

        // 3. Marker Phase (Symbiote)
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Refill roots
        for &root_id in &self.roots {
            if root_id < self.objects.len() && self.objects[root_id].active {
                queue.push_back(root_id);
                visited.insert(root_id);
                // Mark the object on grid
                let obj = &self.objects[root_id];
                let idx = get_idx(obj.x, obj.y);
                self.grid[idx] = FungalCell {
                    fungus_type: FungalType::Marker,
                    age: self.grid[idx].age + 1,
                    energy: 1.0,
                };
            }
        }

        // Propagate Mark
        while let Some(curr_id) = queue.pop_front() {
             if curr_id >= self.objects.len() { continue; }

             // Clone needed data to avoid borrowing self
             let (x0, y0, refs) = {
                 let obj = &self.objects[curr_id];
                 (obj.x, obj.y, obj.references.clone())
             };

             // Mark current spot (refresh energy)
             let idx = get_idx(x0, y0);
             self.grid[idx].energy = 1.0;
             self.grid[idx].fungus_type = FungalType::Marker;

             for child_id in refs {
                 if child_id < self.objects.len() && self.objects[child_id].active {
                     // Get child coords
                     let (x1, y1) = {
                         let child = &self.objects[child_id];
                         (child.x, child.y)
                     };

                     // Now we can mutate self (draw hyphae)
                     self.draw_hypha(x0, y0, x1, y1);

                     if !visited.contains(&child_id) {
                         visited.insert(child_id);
                         queue.push_back(child_id);
                     }
                 }
             }
        }

        // Update logical marked state for all objects
        for (i, obj) in self.objects.iter_mut().enumerate() {
            obj.marked = visited.contains(&i);
        }

        // 4. Sweeper Phase (Decomposer)
        // Spawns randomly in empty space
        for _ in 0..50 {
            let idx = rng.gen_range(0..self.grid.len());
            if self.grid[idx].fungus_type == FungalType::None && rng.gen_bool(0.01) {
                self.grid[idx] = FungalCell {
                    fungus_type: FungalType::Sweeper,
                    age: 0,
                    energy: 0.8,
                };
            }
        }

        // Grow Sweepers
        let mut next_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = get_idx(x, y);
                let cell = self.grid[idx];

                if cell.fungus_type == FungalType::Sweeper && cell.energy > 0.1 {
                    // Try to grow to neighbors
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                                let n_idx = get_idx(nx as usize, ny as usize);
                                let neighbor = self.grid[n_idx];

                                // Rule: Sweeper cannot grow on Marker
                                if neighbor.fungus_type == FungalType::Marker {
                                    // Die back slightly (repelled)
                                    next_grid[idx].energy -= 0.1;
                                } else if neighbor.fungus_type == FungalType::None {
                                    // Spread
                                    if rng.gen_bool(0.2) {
                                        next_grid[n_idx] = FungalCell {
                                            fungus_type: FungalType::Sweeper,
                                            age: 0,
                                            energy: cell.energy - 0.1,
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.grid = next_grid;

        // Check for eaten objects
        // Use helper closure and separate vector to avoid borrow conflicts
        let mut eaten_indices = Vec::new();
        for (i, obj) in self.objects.iter().enumerate() {
            if obj.active && !obj.marked {
                let idx = get_idx(obj.x, obj.y);
                if self.grid[idx].fungus_type == FungalType::Sweeper {
                    eaten_indices.push(i);
                }
            }
        }

        for i in eaten_indices {
            self.objects[i].active = false;
            let (x, y) = (self.objects[i].x, self.objects[i].y);
            let idx = get_idx(x, y);
            self.grid[idx].energy = 1.0; // Boost sweeper
        }
    }

    fn draw_hypha(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        let target_x = x1 as isize;
        let target_y = y1 as isize;

        let dx = (target_x - x).abs();
        let dy = -(target_y - y).abs();
        let sx = if x < target_x { 1 } else { -1 };
        let sy = if y < target_y { 1 } else { -1 };
        let mut err = dx + dy;
        let width = self.width as isize;
        let height = self.height as isize;

        loop {
            if x >= 0 && x < width && y >= 0 && y < height {
                 let idx = (y * width + x) as usize; // Manual index calc
                 if self.grid[idx].fungus_type != FungalType::Marker {
                     self.grid[idx] = FungalCell {
                        fungus_type: FungalType::Marker,
                        age: 0,
                        energy: 1.0,
                    };
                } else {
                     self.grid[idx].energy = 1.0; // Refresh
                }
            }

            if x == target_x && y == target_y { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup world
    let mut world = World::new(GRID_WIDTH, GRID_HEIGHT);
    // Initial seeds
    for _ in 0..5 { world.allocate(); }

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);
    let mut paused = false;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Heap Fungus: Mark & Sweep "))
                .x_bounds([0.0, GRID_WIDTH as f64])
                .y_bounds([0.0, GRID_HEIGHT as f64])
                .paint(|ctx| {
                    let mut marker_pts = Vec::new();
                    let mut sweeper_pts = Vec::new();

                    for y in 0..world.height {
                        for x in 0..world.width {
                            let idx = world.get_index(x, y);
                            let cell = world.grid[idx];
                            if cell.fungus_type != FungalType::None {
                                let pt = (x as f64, (world.height - 1 - y) as f64);
                                match cell.fungus_type {
                                    FungalType::Marker => marker_pts.push(pt),
                                    FungalType::Sweeper => sweeper_pts.push(pt),
                                    _ => {},
                                }
                            }
                        }
                    }
                    ctx.draw(&Points { coords: &marker_pts, color: Color::Cyan });
                    ctx.draw(&Points { coords: &sweeper_pts, color: Color::Red });

                    // Draw Objects
                    for obj in &world.objects {
                        if obj.active {
                            let color = if obj.marked { Color::Green } else { Color::Yellow };
                            let rect = Rectangle {
                                x: obj.x as f64 - obj.size as f64 / 2.0,
                                y: (world.height - 1 - obj.y) as f64 - obj.size as f64 / 2.0,
                                width: obj.size as f64,
                                height: obj.size as f64,
                                color,
                            };
                            ctx.draw(&rect);
                        }
                    }
                });
            f.render_widget(canvas, area);
        })?;

        // Input
        if event::poll(Duration::from_millis(10))? {
             if let Event::Key(key) = event::read()? {
                 match key.code {
                     KeyCode::Char('q') => break,
                     KeyCode::Char('a') => world.allocate(),
                     KeyCode::Char('d') => world.mutate_references(),
                     KeyCode::Char('r') => world = World::new(GRID_WIDTH, GRID_HEIGHT),
                     KeyCode::Char(' ') => paused = !paused,
                     _ => {}
                 }
             }
        }

        // Update
        if !paused && last_tick.elapsed() >= tick_rate {
            world.update();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_spread() {
        let mut world = World::new(20, 20);

        // Create Object A (Root)
        world.objects.push(Object {
            id: 0, x: 2, y: 2, size: 1, references: vec![1], active: true, marked: false
        });
        world.roots.push(0);

        // Create Object B (Child)
        world.objects.push(Object {
            id: 1, x: 2, y: 10, size: 1, references: vec![], active: true, marked: false
        });

        // Run update multiple times to allow propagation
        for _ in 0..5 {
            world.update();
        }

        assert!(world.objects[0].marked, "Root should be marked");
        assert!(world.objects[1].marked, "Child should be marked via hyphae");

        // Check grid for Hyphae
        let mid_idx = world.get_index(2, 6);
        assert_eq!(world.grid[mid_idx].fungus_type, FungalType::Marker, "Hyphae should exist between objects");
    }

    #[test]
    fn test_sweeper_eating() {
        let mut world = World::new(20, 20);

        // Create Object A (Garbage, not root)
        world.objects.push(Object {
            id: 0, x: 10, y: 10, size: 1, references: vec![], active: true, marked: false
        });

        // Manually place Sweeper on top of it
        let idx = world.get_index(10, 10);
        world.grid[idx] = FungalCell {
            fungus_type: FungalType::Sweeper,
            age: 0,
            energy: 1.0,
        };

        // Run update
        world.update();

        assert!(!world.objects[0].active, "Garbage object should be eaten by Sweeper");
    }

    #[test]
    fn test_sweeper_safety() {
        let mut world = World::new(20, 20);

        // Create Object A (Root, Marked)
        world.objects.push(Object {
            id: 0, x: 10, y: 10, size: 1, references: vec![], active: true, marked: false
        });
        world.roots.push(0);

        // Manually place Sweeper next to it
        let idx = world.get_index(10, 11);
        world.grid[idx] = FungalCell {
            fungus_type: FungalType::Sweeper,
            age: 0,
            energy: 1.0,
        };

        // Run update
        world.update();

        assert!(world.objects[0].active, "Marked object should NOT be eaten");
        assert!(world.objects[0].marked, "Object should be marked");
    }
}
