use crate::allocator::Heap;
use crate::level_gen::{BlockType, BossStats, LevelProfile};
use rand::Rng;

pub const GRAVITY: f64 = -0.15;
pub const JUMP_FORCE: f64 = 1.2;
pub const MOVE_SPEED: f64 = 0.8;
pub const MAX_SPEED: f64 = 1.5;
pub const FRICTION: f64 = 0.85;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub can_jump: bool,
    pub hp: i32,
    pub max_hp: i32,
    pub is_dead: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 2.0,
            y: 5.0,
            vx: 0.0,
            vy: 0.0,
            can_jump: false,
            hp: 100,
            max_hp: 100,
            is_dead: false,
        }
    }

    pub fn jump(&mut self) {
        if self.can_jump {
            self.vy = JUMP_FORCE;
            self.can_jump = false;
        }
    }

    pub fn move_left(&mut self) {
        self.vx = (self.vx - MOVE_SPEED).max(-MAX_SPEED);
    }

    pub fn move_right(&mut self) {
        self.vx = (self.vx + MOVE_SPEED).min(MAX_SPEED);
    }
}

pub struct Boss {
    pub x: f64,
    pub y: f64,
    pub stats: BossStats,
    pub cooldown: u32,
}

pub struct Projectile {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub symbol: char,
}

pub struct Game {
    pub heap: Heap,
    pub player: Player,
    pub boss: Boss,
    pub projectiles: Vec<Projectile>,
    pub score: u64,
    pub game_over: bool,
    pub win: bool,
    pub time: u64,
    pub scroll_offset: f64,
    pub messages: Vec<String>,
}

impl Game {
    pub fn new(profile: LevelProfile) -> Self {
        let heap = Heap::from_profile(&profile);
        let boss_x = heap.total_size as f64 - 20.0;

        Self {
            heap,
            player: Player::new(),
            boss: Boss {
                x: boss_x,
                y: 10.0,
                stats: profile.boss,
                cooldown: 0,
            },
            projectiles: Vec::new(),
            score: 0,
            game_over: false,
            win: false,
            time: 0,
            scroll_offset: 0.0,
            messages: vec![
                "WELCOME TO THE HEAP ARENA".to_string(),
                "Collect the function!".to_string(),
            ],
        }
    }

    pub fn tick(&mut self) {
        if self.game_over || self.win {
            return;
        }

        self.time += 1;

        // --- Player Physics ---
        self.player.vy += GRAVITY;
        self.player.x += self.player.vx;
        self.player.y += self.player.vy;

        // Friction
        self.player.vx *= FRICTION;
        if self.player.vx.abs() < 0.01 {
            self.player.vx = 0.0;
        }

        // Bounds
        if self.player.x < 0.0 {
            self.player.x = 0.0;
            self.player.vx = 0.0;
        }

        // --- Collision ---
        // Floor Check
        if self.player.y <= 0.0 {
            // Find block under player
            let center_x = self.player.x as usize;
            let mut landed = false;
            let mut hit_hazard = false;
            let mut hit_bouncy = false;

            // Inefficient search but heap is sorted by start, so we could optimize
            // But strict linear search is fine for < 500 blocks.
            for block in &self.heap.blocks {
                if center_x >= block.start && center_x < (block.start + block.size) {
                    if block.is_solid {
                        landed = true;
                        if matches!(block.block_type, BlockType::Hazard) {
                            hit_hazard = true;
                        }
                        if matches!(block.block_type, BlockType::Bouncy) {
                            hit_bouncy = true;
                        }
                    }
                    break;
                }
            }

            if landed {
                self.player.y = 0.0;

                if hit_bouncy {
                    self.player.vy = JUMP_FORCE * 1.5; // Auto bounce
                    self.player.can_jump = false;
                } else {
                    self.player.vy = 0.0;
                    self.player.can_jump = true;
                }

                if hit_hazard && self.time % 10 == 0 {
                    self.player.hp -= 5;
                    self.messages.push("OUCH! Unsafe code!".to_string());
                }
            } else {
                // Falling into void
                self.player.can_jump = false;
            }
        }

        // Death Check (Void)
        if self.player.y < -20.0 {
            self.player.is_dead = true;
            self.game_over = true;
            self.messages.push("SEGFAULT (Core Dumped)".to_string());
        }

        // Death Check (HP)
        if self.player.hp <= 0 {
            self.player.is_dead = true;
            self.game_over = true;
            self.messages.push("Killed by Complexity!".to_string());
        }

        // Win Check
        if self.player.x >= (self.heap.total_size as f64 - 10.0) {
            self.win = true;
            self.score += 1000;
            self.messages.push("FUNCTION COLLECTED!".to_string());
        }

        // --- Boss Logic ---
        // Boss moves slowly towards player or hovers?
        // Let's make boss hover at end, but shoot.

        if self.boss.cooldown > 0 {
            self.boss.cooldown -= 1;
        } else {
            // Attack logic based on distance
            let dist = (self.boss.x - self.player.x).abs();
            if dist < 80.0 && !self.player.is_dead {
                // Shoot
                let mut rng = rand::thread_rng();
                // Higher Attack = More frequent shots or more projectiles?
                // Let's say cooldown is based on attack?
                // Base cooldown 60. Attack reduces it.
                let attack_mod = (self.boss.stats.attack / 10).clamp(0, 40);
                self.boss.cooldown = 60 - attack_mod as u32;

                let angle = rng.gen_range(std::f64::consts::PI * 0.8..std::f64::consts::PI * 1.2);
                let speed = 0.5 + (self.boss.stats.speed as f64 / 20.0);

                self.projectiles.push(Projectile {
                    x: self.boss.x,
                    y: self.boss.y,
                    vx: angle.cos() * speed * -1.0, // Left
                    vy: angle.sin() * speed,        // Up/Down
                    symbol: if rng.gen_bool(0.5) { 'E' } else { '!' }, // E for Error/Exception
                });
            }
        }

        // --- Projectiles ---
        let mut to_remove = Vec::new();
        for (i, p) in self.projectiles.iter_mut().enumerate() {
            p.x += p.vx;
            p.y += p.vy;
            p.vy += GRAVITY * 0.1; // Slight gravity for projectiles

            if p.y < -10.0 || p.x < self.scroll_offset - 10.0 {
                to_remove.push(i);
                continue;
            }

            // Hit Player?
            if (p.x - self.player.x).abs() < 1.0 && (p.y - self.player.y).abs() < 1.0 {
                self.player.hp -= 10;
                self.messages.push("Hit by Exception!".to_string());
                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            self.projectiles.remove(i);
        }

        // --- Scroll ---
        // Keep player in middle-left
        let target_scroll = self.player.x - 20.0;
        if target_scroll > self.scroll_offset {
            self.scroll_offset = target_scroll;
        }
    }
}
