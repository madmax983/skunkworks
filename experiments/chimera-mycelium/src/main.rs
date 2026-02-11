use chimera_lang::prelude::*;
use macroquad::prelude::*;
use ::rand::Rng;

const GRID_SIZE: usize = 40;

#[derive(Clone)]
pub struct Spore {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub dna: Dna,
}

impl Spore {
    pub fn new(pos: Vec2, color: Color, dna: Dna) -> Self {
        let mut rng = ::rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(0.5..2.0);
        Self {
            pos,
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color,
            life: rng.gen_range(5.0..15.0), // Longer life for genetic material
            max_life: 15.0,
            dna,
        }
    }

    pub fn update(&mut self, dt: f32, wind: Vec2) {
        self.vel += wind * dt * 5.0; // Wind influence
        self.vel *= 0.98; // Friction
        self.pos += self.vel;
        self.life -= dt;
    }
}

pub struct ChimeraNode {
    pub vm: ChimeraVM,
    pub pos: Vec2,
    pub radius: f32,
    pub color: Color,
    pub pulse: f32,
    pub last_spore_tick: usize,
}

impl ChimeraNode {
    pub fn new(pos: Vec2, dna: Dna) -> Self {
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 50; // Starting energy
        Self {
            vm,
            pos,
            radius: 10.0,
            color: WHITE,
            pulse: 0.0,
            last_spore_tick: 0,
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<Vec<Spore>> {
        self.pulse += dt * (1.0 + (self.vm.energy as f32 * 0.01));

        // Step the VM
        // Execute a few ticks per frame
        for _ in 0..5 {
            if self.vm.energy > 0 {
                self.vm.step();
            } else {
                // Recover slowly if dead? Or stay dead?
                // Let's say they can photosynthesize slowly if dead
                self.vm.energy += 1;
            }
        }

        // Color based on DNA hash (simplified)
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        self.vm.dna.hash(&mut hasher);
        let hash = hasher.finish();
        let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
        let b = (hash & 0xFF) as f32 / 255.0;
        self.color = Color::new(r, g, b, 1.0);

        // Check for SporeCloud OpCode execution or high energy
        let mut spores = Vec::new();

        // Automatic sporulation if high energy
        if self.vm.energy > 200 && (self.vm.tick_counter as usize) - self.last_spore_tick > 50 {
            self.vm.energy -= 50; // Cost of reproduction
            self.last_spore_tick = self.vm.tick_counter as usize;

            let count = 5;
            for _ in 0..count {
                // Create a mutated copy of DNA
                let mut child_dna = self.vm.dna.clone();
                // Simple mutation: swap two genes in a random strand
                let mut rng = ::rand::thread_rng();
                if let Some(strand) = child_dna.helix.strands.get_mut(0) {
                     if strand.genes.len() > 1 {
                         let i = rng.gen_range(0..strand.genes.len());
                         let j = rng.gen_range(0..strand.genes.len());
                         strand.genes.swap(i, j);
                     }
                }

                spores.push(Spore::new(self.pos, self.color, child_dna));
            }
            return Some(spores);
        }

        None
    }

    pub fn absorb_spore(&mut self, spore: &Spore) {
        // Horizontal Gene Transfer
        // Incorporate spore DNA into self
        // For simplicity, just replace the second strand, or append if only 1
        if self.vm.dna.helix.strands.len() < 2 {
            if let Some(new_strand) = spore.dna.helix.strands.first() {
                self.vm.dna.helix.strands.push(new_strand.clone());
            }
        } else {
            // Replace a random strand (not the first one, keep core identity)
            let mut rng = ::rand::thread_rng();
            let idx = rng.gen_range(1..self.vm.dna.helix.strands.len());
            if let Some(new_strand) = spore.dna.helix.strands.first() {
                self.vm.dna.helix.strands[idx] = new_strand.clone();
            }
        }
        self.vm.energy += 20; // Food value
    }
}

pub struct World {
    pub nodes: Vec<ChimeraNode>,
    pub spores: Vec<Spore>,
    pub wind_grid: [[Vec2; GRID_SIZE]; GRID_SIZE],
    pub time: f64,
    pub width: f32,
    pub height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        let mut wind_grid = [[Vec2::ZERO; GRID_SIZE]; GRID_SIZE];
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                wind_grid[y][x] = vec2(0.0, 0.0);
            }
        }

        Self {
            nodes: Vec::new(),
            spores: Vec::new(),
            wind_grid,
            time: 0.0,
            width,
            height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt as f64;

        // Update wind
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let fx = x as f64 * 0.1 + self.time * 0.5;
                let fy = y as f64 * 0.1 + self.time * 0.2;
                let angle = (fx.sin() + fy.cos()) * std::f64::consts::PI;
                self.wind_grid[y][x] = vec2(angle.cos() as f32, angle.sin() as f32) * 0.5;
            }
        }

        // Process Spores & Collisions
        let mut hits: Vec<(usize, usize)> = Vec::new(); // (node_index, spore_index)
        let width = self.width;
        let height = self.height;
        let wind_grid = &self.wind_grid;

        // Move spores
        for spore in &mut self.spores {
            let gx = (spore.pos.x / width * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32);
            let gy = (spore.pos.y / height * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32);
            let wind = wind_grid[gy as usize][gx as usize];
            spore.update(dt, wind);
        }

        // Check collisions
        for (si, spore) in self.spores.iter().enumerate() {
            if spore.life <= 0.0 || spore.pos.x <= 0.0 || spore.pos.x >= width || spore.pos.y <= 0.0 || spore.pos.y >= height {
                continue; // Will be removed later
            }

            for (ni, node) in self.nodes.iter().enumerate() {
                 if spore.pos.distance_squared(node.pos) < node.radius * node.radius {
                     hits.push((ni, si));
                     break; // One hit per spore
                 }
            }
        }

        // Apply hits (Transfer DNA)
        // Need to be careful with indices since we are removing spores
        // Sort hits by spore index descending to remove correctly?
        // Actually, let's just mark spores for removal.
        let mut dead_spores = std::collections::HashSet::new();

        for (ni, si) in hits {
            if dead_spores.contains(&si) { continue; }
            let spore = self.spores[si].clone();
            self.nodes[ni].absorb_spore(&spore);
            dead_spores.insert(si);
        }

        // Remove dead spores
        let mut i = 0;
        self.spores.retain(|_| {
            let retain = !dead_spores.contains(&i);
            i += 1;
            retain
        });

        // Remove old/out of bounds spores
        self.spores.retain(|s| s.life > 0.0 && s.pos.x > 0.0 && s.pos.x < width && s.pos.y > 0.0 && s.pos.y < height);

        // Update Nodes and spawn spores
        let mut new_spores = Vec::new();
        for node in &mut self.nodes {
            if let Some(mut spores) = node.update(dt) {
                new_spores.append(&mut spores);
            }
        }
        self.spores.append(&mut new_spores);
    }
}

fn random_dna() -> Dna {
    let mut rng = ::rand::thread_rng();
    let ops = vec![
        OpCode::Push, OpCode::Drop, OpCode::Add, OpCode::Sub,
        OpCode::Photosynthesize, OpCode::Consume,
        OpCode::GRead, OpCode::GWrite, OpCode::Radiate,
        OpCode::SporeCloud, OpCode::Hyphae,
        OpCode::Jump, OpCode::Dup, OpCode::Swap
    ];

    let mut genes = Vec::new();
    for _ in 0..10 {
        let op = ops[rng.gen_range(0..ops.len())].clone();
        let args = vec![Nucleotide::Number(rng.gen_range(0..10))];
        genes.push(Gene { op, args });
    }

    Dna { helix: Helix { strands: vec![Strand { genes }] } }
}

#[macroquad::main("Chimera Mycelium")]
async fn main() {
    let mut world = World::new(screen_width(), screen_height());

    // Add some initial nodes
    for _ in 0..5 {
        let mut rng = ::rand::thread_rng();
        let pos = vec2(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height()));
        world.nodes.push(ChimeraNode::new(pos, random_dna()));
    }

    loop {
        let dt = get_frame_time();

        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            world.nodes.push(ChimeraNode::new(vec2(mpos.0, mpos.1), random_dna()));
        }

        if is_key_pressed(KeyCode::R) {
             world = World::new(screen_width(), screen_height());
             for _ in 0..5 {
                let mut rng = ::rand::thread_rng();
                let pos = vec2(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height()));
                world.nodes.push(ChimeraNode::new(pos, random_dna()));
            }
        }

        // Update
        world.update(dt);

        // Draw
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        // Draw Wind (Subtle)
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let wind = world.wind_grid[y][x];
                let px = (x as f32 / GRID_SIZE as f32) * screen_width();
                let py = (y as f32 / GRID_SIZE as f32) * screen_height();
                draw_line(px, py, px + wind.x * 20.0, py + wind.y * 20.0, 1.0, Color::new(0.2, 0.2, 0.2, 0.1));
            }
        }

        // Draw Spores
        for spore in &world.spores {
            let alpha = (spore.life / spore.max_life).clamp(0.0, 1.0);
            let color = Color::new(spore.color.r, spore.color.g, spore.color.b, alpha);
            draw_circle(spore.pos.x, spore.pos.y, 2.0, color);
        }

        // Draw Nodes
        for node in &world.nodes {
            let pulse_scale = 1.0 + (node.pulse.sin() * 0.1);
            let energy_glow = (node.vm.energy as f32 / 500.0).clamp(0.0, 1.0);

            // Draw connections (Hyphae) - Simplified as lines to neighbors?
            // Expensive O(N^2), maybe skip for now or do localized check

            draw_circle(node.pos.x, node.pos.y, node.radius * pulse_scale, node.color);
            draw_circle_lines(node.pos.x, node.pos.y, node.radius * pulse_scale + energy_glow * 5.0, 2.0, WHITE);

            // Draw Stats
            // draw_text(&format!("{}", node.vm.energy), node.pos.x - 10.0, node.pos.y - 15.0, 15.0, WHITE);
        }

        draw_text(format!("Nodes: {}", world.nodes.len()).as_str(), 10.0, 20.0, 20.0, WHITE);
        draw_text(format!("Spores: {}", world.spores.len()).as_str(), 10.0, 40.0, 20.0, WHITE);
        draw_text("Click: Add Node | R: Reset", 10.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await
    }
}
