#![allow(clippy::manual_is_multiple_of)]
use crate::allocator::Heap;
use rand::Rng;

pub const GRAVITY: f64 = -0.15;
pub const JUMP_FORCE: f64 = 1.5;
pub const MOVE_SPEED: f64 = 1.0;
pub const MAX_SPEED: f64 = 2.0;
pub const FRICTION: f64 = 0.8;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub can_jump: bool,
    pub is_dead: bool,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            can_jump: false,
            is_dead: false,
        }
    }

    pub fn move_left(&mut self) {
        self.vx = (self.vx - MOVE_SPEED).max(-MAX_SPEED);
    }

    pub fn move_right(&mut self) {
        self.vx = (self.vx + MOVE_SPEED).min(MAX_SPEED);
    }
}

pub struct Game {
    pub heap: Heap,
    pub player: Player,
    pub score: u64,
    pub game_over: bool,
    pub time: u64,
    pub scroll_offset: f64,
}

impl Game {
    pub fn new(heap_size: usize) -> Self {
        let mut heap = Heap::new(heap_size);
        // Initial allocation to stand on
        heap.allocate(20); // Start block
        heap.allocate(10); // Gap? No, allocate fills first.

        // We want some initial terrain.
        // Let's manually set it up or run some random allocs.
        // For now, let's just make the first 20 units allocated.

        Self {
            heap,
            player: Player::new(5.0, 0.0), // Start on the first block
            score: 0,
            game_over: false,
            time: 0,
            scroll_offset: 0.0,
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        self.time += 1;
        self.score += 1;

        // 1. Update Physics
        self.player.vy += GRAVITY;
        self.player.x += self.player.vx;
        self.player.y += self.player.vy;

        // Friction
        self.player.vx *= FRICTION;
        if self.player.vx.abs() < 0.01 {
            self.player.vx = 0.0;
        }

        // Clamp X to heap bounds
        let max_x = self.heap.total_size as f64;
        if self.player.x < 0.0 {
            self.player.x = 0.0;
            self.player.vx = 0.0;
        }
        if self.player.x >= max_x {
            self.player.x = max_x - 0.1;
            self.player.vx = 0.0;
        }

        // 2. Collision Detection
        if self.player.y <= 0.0 {
            // Check if we are on solid ground
            let x_idx = self.player.x as usize;

            // Find block at x_idx
            let mut is_solid = false;
            let mut current_pos = 0;
            for block in &self.heap.blocks {
                let end_pos = current_pos + block.size;
                if x_idx >= current_pos && x_idx < end_pos {
                    if block.is_allocated {
                        is_solid = true;
                    }
                    break;
                }
                current_pos = end_pos;
            }

            if is_solid {
                // Landed
                self.player.y = 0.0;
                self.player.vy = 0.0;
                self.player.can_jump = true;
            } else {
                // Falling into void
                self.player.can_jump = false;
                // No floor clamping
            }
        }

        // Death Check
        if self.player.y < -10.0 {
            self.game_over = true;
            self.player.is_dead = true;
        }

        // 3. Allocator Events
        // Every N ticks, do something
        let mut rng = rand::thread_rng();
        if self.time % 20 == 0 {
            // Randomly free something
            if rng.gen_bool(0.3) {
                self.heap.random_free();
            }

            // Randomly allocate something
            if rng.gen_bool(0.6) {
                let size = rng.gen_range(5..15);
                self.heap.allocate(size);
            }
        }

        // Keep initial block safe? No, let chaos reign.
    }

    pub fn jump(&mut self) {
        if self.player.can_jump {
            self.player.vy = JUMP_FORCE;
            self.player.can_jump = false;
        }
    }
}
