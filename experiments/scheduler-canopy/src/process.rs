use macroquad::prelude::*;
use crate::cpu::Scheduler;
use crate::memory::{MemoryGrid, Pos};

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Zombie, // OOM or Terminated
}

pub struct ProcessTree {
    pub pid: usize,
    pub priority: u8,
    pub state: ProcessState,

    // Position
    pub position: Vec2, // Base of trunk (World Coords)
    pub angle: f32, // Angle for Sun calculation (e.g. normalized 0..PI across screen width)

    // Resources
    pub energy: f32, // Accumulated CPU time
    pub memory_usage: usize, // Total allocated cells

    // Roots
    pub root_tips: Vec<Pos>, // Current active root tips

    // Canopy (L-System)
    pub l_system_string: String,
    pub generation: usize,
    pub max_depth: usize,
    pub branches: Vec<Branch>, // Pre-calculated geometry for rendering
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
    // pub color: Color,
}

impl ProcessTree {
    pub fn new(pid: usize, position: Vec2, screen_width: f32) -> Self {
        // Map X position to 0..PI for sun angle
        let angle = (position.x / screen_width) * std::f32::consts::PI;

        Self {
            pid,
            priority: 1,
            state: ProcessState::Sleeping,
            position,
            angle,
            energy: 0.0,
            memory_usage: 0,
            root_tips: Vec::new(), // Initialized later or passed in?
                                   // Actually, roots need to start somewhere in the grid corresponding to world pos.
            l_system_string: "F".to_string(),
            generation: 0,
            max_depth: 4,
            branches: Vec::new(),
        }
    }

    /// Initialize roots at the corresponding grid position
    pub fn init_roots(&mut self, memory: &mut MemoryGrid) {
        // Map world X to grid X. World Y (base) is usually 0 or center.
        // Assume memory grid covers the bottom half.
        // Let's assume grid width matches screen width scaling.
        // But memory grid is discrete.

        let grid_x = ((self.position.x / screen_width()) * memory.width as f32) as usize;
        let grid_y = 0; // Top of memory grid (which is rendered at bottom of screen)

        // Try to allocate initial seed
        let start_pos = Pos { x: grid_x.clamp(0, memory.width-1), y: grid_y };
        let allocs = memory.allocate_from(&vec![start_pos], 1, self.pid);

        if !allocs.is_empty() {
            self.root_tips = allocs;
            self.memory_usage = 1;
            self.rebuild_geometry();
        } else {
            // Failed to plant seed!
            self.state = ProcessState::Zombie;
        }
    }

    pub fn update(&mut self, scheduler: &Scheduler, memory: &mut MemoryGrid, dt: f32) {
        if let ProcessState::Zombie = self.state {
            return;
        }

        // 1. Check CPU (Sun)
        if scheduler.is_active(self.angle) {
            self.state = ProcessState::Running;
            self.energy += dt * (self.priority as f32);
        } else {
            self.state = ProcessState::Sleeping;
        }

        // 2. Growth Logic
        // Cost to grow one L-system iteration:
        // Needs (Generation * 10) energy?
        // Needs (Generation * 5) memory cells?
        let memory_needed = (self.l_system_string.len() / 2).max(1);

        if self.state == (ProcessState::Running) && self.energy > 1.0 {
            // Try to grow roots first if memory insufficient
            if self.memory_usage < memory_needed {
                // Try to allocate
                let new_allocs = memory.allocate_from(&self.root_tips, 1, self.pid); // Grow 1 at a time
                if !new_allocs.is_empty() {
                    self.memory_usage += new_allocs.len();
                    // Update tips: remove old tip if it branched?
                    // Or just append. Simplest: append.
                    // For a tree structure, we might want to replace the parent tip if it's "used up",
                    // but MemoryGrid::allocate_from doesn't consume tips.
                    // Let's just keep the new ones as the active frontier.
                    // Maybe keep *some* old ones? Randomly?
                    // Let's just replace tips with new allocs to simulate digging down.
                    // Wait, if we replace, we lose ability to branch from old.
                    // Let's append new ones, but limit total tips to avoid explosion.
                    self.root_tips.extend(new_allocs);
                    if self.root_tips.len() > 10 {
                        self.root_tips.drain(0..5); // Remove oldest
                    }
                    self.energy -= 0.5; // Root growth costs energy
                } else {
                    // OOM!
                    // If repeated failures?
                }
            } else {
                // Have enough memory, grow canopy
                if self.generation < self.max_depth {
                    self.grow_l_system();
                    self.energy -= 1.0; // Canopy growth costs energy
                }
            }
        }
    }

    fn grow_l_system(&mut self) {
        let mut next = String::new();
        for c in self.l_system_string.chars() {
            match c {
                'F' => next.push_str("FF+[+F-F-F]-[-F+F+F]"), // Basic bush rule
                _ => next.push(c),
            }
        }
        self.l_system_string = next;
        self.generation += 1;
        self.rebuild_geometry();
    }

    fn rebuild_geometry(&mut self) {
        self.branches.clear();
        let mut stack = Vec::new();
        let mut pos = self.position;
        let mut angle = -std::f32::consts::PI / 2.0; // Up
        let len = 20.0 / (self.generation as f32 + 1.0); // Shrink as we grow?

        for c in self.l_system_string.chars() {
            match c {
                'F' => {
                    let end = pos + vec2(angle.cos(), angle.sin()) * len;
                    self.branches.push(Branch {
                        start: pos,
                        end,
                        thickness: (self.max_depth - self.generation) as f32 + 1.0,
                    });
                    pos = end;
                }
                '+' => angle += 0.4, // ~25 deg
                '-' => angle -= 0.4,
                '[' => stack.push((pos, angle)),
                ']' => {
                    if let Some((p, a)) = stack.pop() {
                        pos = p;
                        angle = a;
                    }
                }
                _ => {}
            }
        }
    }
}
