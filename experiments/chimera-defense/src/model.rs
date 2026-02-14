use std::collections::VecDeque;
use chimera_lang::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 24;

#[derive(Debug, Clone, Copy)]
pub struct LogisticMap {
    pub x: f64,
    pub r: f64,
}

impl LogisticMap {
    pub fn new(x: f64, r: f64) -> Self {
        Self { x, r }
    }

    pub fn next(&mut self) -> f64 {
        self.x = self.r * self.x * (1.0 - self.x);
        self.x
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub pos: Position,
    pub hp: f64,
    pub id: usize,
    pub chaos_val: f64,
}

#[derive(Clone)]
pub struct Tower {
    pub pos: Position,
    pub cooldown: usize,
    pub vm: ChimeraVM,
    pub kills: usize,
}

#[derive(Debug, Clone)]
pub struct Projectile {
    pub start: Position,
    pub end: Position,
    pub progress: f64,
}

#[derive(Debug, Clone)]
pub struct Nest {
    pub pos: Position,
    pub chaos: LogisticMap,
    pub spawn_timer: f64,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub enemies: Vec<Enemy>,
    pub towers: Vec<Tower>,
    pub nests: Vec<Nest>,
    pub projectiles: Vec<Projectile>,
    pub global_r: f64,
    pub resources: f64,
    pub ticks: usize,
    pub history: VecDeque<f64>,
}

impl Tower {
    pub fn new(pos: Position) -> Self {
        // Default DNA: Simple Fire Loop
        // 0: push(1) (Fire Value)
        // 1: push(0) (X)
        // 2: push(15) (Y)
        // 3: g_write()
        // 4: jump(0)
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] },
            Gene { op: OpCode::GWrite, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        Self {
            pos,
            cooldown: 0,
            vm: ChimeraVM::new(dna),
            kills: 0,
        }
    }

    pub fn tick(&mut self, enemy_info: Option<(f64, f64)>) -> Option<bool> {
        // 1. Write Sensors
        if let Some((dist, angle)) = enemy_info {
            self.vm.grid[0][0] = Value::Int(dist as i64);
            self.vm.grid[0][1] = Value::Int((angle * 100.0) as i64); // Scale angle
        } else {
            self.vm.grid[0][0] = Value::Int(9999);
        }

        // 2. Step VM
        for _ in 0..10 {
            if !self.vm.halted {
                self.vm.step();
            }
        }

        // 3. Read Actuators (Grid 15,0 for Fire)
        if let Value::Int(val) = self.vm.grid[15][0] {
            if val > 0 {
                // Reset actuator
                self.vm.grid[15][0] = Value::Int(0);
                return Some(true); // Fire!
            }
        }

        None
    }
}

impl World {
    pub fn new() -> Self {
        let nests = vec![
            Nest {
                pos: Position { x: 5.0, y: 5.0 },
                chaos: LogisticMap::new(0.5, 3.0),
                spawn_timer: 0.0,
            },
            Nest {
                pos: Position { x: 75.0, y: 20.0 },
                chaos: LogisticMap::new(0.6, 3.2),
                spawn_timer: 0.0,
            },
             Nest {
                pos: Position { x: 40.0, y: 2.0 },
                chaos: LogisticMap::new(0.1, 3.9),
                spawn_timer: 0.0,
            }
        ];

        Self {
            width: WIDTH,
            height: HEIGHT,
            enemies: Vec::new(),
            towers: Vec::new(),
            nests,
            projectiles: Vec::new(),
            global_r: 3.0,
            resources: 100.0,
            ticks: 0,
            history: VecDeque::with_capacity(200),
        }
    }

    pub fn add_tower(&mut self, x: f64, y: f64) {
        if self.resources >= 20.0 {
            self.resources -= 20.0;
            self.towers.push(Tower::new(Position { x, y }));
            // Adding a tower increases industrialization -> higher r
            self.global_r += 0.05;
        }
    }

    pub fn update(&mut self) {
        self.ticks += 1;

        // Influence nests with global_r
        for (i, nest) in self.nests.iter_mut().enumerate() {
            let drift = (self.global_r - nest.chaos.r) * 0.005;
            nest.chaos.r += drift;
            nest.chaos.r = nest.chaos.r.clamp(0.0, 4.0);

            // Tick chaos
            nest.chaos.next();

            // Record history from the first nest
            if i == 0 {
                self.history.push_back(nest.chaos.x);
                if self.history.len() > 200 {
                    self.history.pop_front();
                }
            }

            // Spawn logic
            nest.spawn_timer += nest.chaos.x;
            if nest.spawn_timer > 5.0 {
                nest.spawn_timer = 0.0;
                self.enemies.push(Enemy {
                    pos: nest.pos,
                    hp: 10.0,
                    id: self.ticks,
                    chaos_val: nest.chaos.x,
                });
            }
        }

        // Move enemies
        let center = Position { x: (WIDTH / 2) as f64, y: (HEIGHT / 2) as f64 };
        for enemy in &mut self.enemies {
            let dx = center.x - enemy.pos.x;
            let dy = center.y - enemy.pos.y;
            let dist = (dx*dx + dy*dy).sqrt();

            // Deterministic jitter
            enemy.chaos_val = 3.9 * enemy.chaos_val * (1.0 - enemy.chaos_val);
            let angle_offset = (enemy.chaos_val - 0.5) * 1.0;

            if dist > 0.5 {
                let speed = 0.3;
                let base_angle = dy.atan2(dx);
                let final_angle = base_angle + angle_offset;

                enemy.pos.x += final_angle.cos() * speed;
                enemy.pos.y += final_angle.sin() * speed;
            } else {
                self.resources -= 5.0;
                enemy.hp = 0.0;
            }
        }

        self.enemies.retain(|e| e.hp > 0.0);

        // Update Projectiles
        let mut hit_indices = Vec::new();
        let mut dead_projectiles = Vec::new();

        for (p_idx, proj) in self.projectiles.iter_mut().enumerate() {
            proj.progress += 0.2; // Speed
            if proj.progress >= 1.0 {
                dead_projectiles.push(p_idx);
                // Check collision at end
                for (e_idx, enemy) in self.enemies.iter().enumerate() {
                     let dx = enemy.pos.x - proj.end.x;
                     let dy = enemy.pos.y - proj.end.y;
                     if (dx*dx + dy*dy).sqrt() < 2.0 {
                         hit_indices.push(e_idx);
                     }
                }
            }
        }

        // Clean projectiles
        for idx in dead_projectiles.iter().rev() {
            self.projectiles.remove(*idx);
        }

        // Apply Damage
        for idx in hit_indices {
            if idx < self.enemies.len() {
                self.enemies[idx].hp -= 5.0;
                 if self.enemies[idx].hp <= 0.0 {
                     self.resources += 2.0;
                 }
            }
        }
        self.enemies.retain(|e| e.hp > 0.0);

        // Update Towers
        for tower in &mut self.towers {
             if tower.cooldown > 0 {
                tower.cooldown -= 1;
                // Still tick VM? Yes, let it think.
                tower.tick(None);
                continue;
            }

            // Find nearest enemy info
            let mut min_dist = 9999.0;
            let mut best_target_pos = None;

            for enemy in &self.enemies {
                 let dx = enemy.pos.x - tower.pos.x;
                 let dy = enemy.pos.y - tower.pos.y;
                 let dist = (dx*dx + dy*dy).sqrt();
                 if dist < min_dist {
                     min_dist = dist;
                     best_target_pos = Some(enemy.pos);
                 }
            }

            let sensor_info = if let Some(pos) = best_target_pos {
                let dx = pos.x - tower.pos.x;
                let dy = pos.y - tower.pos.y;
                let angle = dy.atan2(dx);
                Some((min_dist, angle))
            } else {
                None
            };

            if let Some(should_fire) = tower.tick(sensor_info) {
                if should_fire {
                     if let Some(target_pos) = best_target_pos {
                         // Spawn projectile
                         self.projectiles.push(Projectile {
                             start: tower.pos,
                             end: target_pos,
                             progress: 0.0,
                         });
                         tower.cooldown = 10;
                     }
                }
            }
        }

        if self.resources < 0.0 {
            self.resources = 0.0;
        }
    }
}
