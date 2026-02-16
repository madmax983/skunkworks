use chimera_lang::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};
use crate::geo::MiuraPattern;
use nalgebra::{Point3, Vector3};

pub const ROWS: usize = 12;
pub const COLS: usize = 12;

#[derive(Clone)]
pub struct Tower {
    pub r: usize,
    pub c: usize,
    pub cooldown: usize,
    pub vm: ChimeraVM,
    pub kills: usize,
    pub range_mult: f64,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub r: usize,
    pub c: usize,
    pub hp: f64,
    pub id: usize,
    pub move_timer: usize,
    pub max_hp: f64,
}

#[derive(Debug, Clone)]
pub struct Projectile {
    pub pos: Point3<f64>,
    pub velocity: Vector3<f64>,
    pub life: usize,
}

pub struct World {
    pub pattern: MiuraPattern,
    pub rho: f64,
    pub target_rho: f64,
    pub enemies: Vec<Enemy>,
    pub towers: Vec<Tower>,
    pub projectiles: Vec<Projectile>,
    pub resources: f64,
    pub ticks: usize,
    pub spawn_timer: usize,
    pub wave: usize,
}

impl Tower {
    pub fn new(r: usize, c: usize) -> Self {
        // Default DNA: Simple Fire Loop
        // 0: push(1) (Fire Value)
        // 1: push(0) (X - unused in grid mode)
        // 2: push(15) (Y - Actuator Address)
        // 3: g_write()
        // 4: jump(0)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(15)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        Self {
            r, c,
            cooldown: 0,
            vm: ChimeraVM::new(dna),
            kills: 0,
            range_mult: 1.0,
        }
    }

    pub fn tick(&mut self, enemy_info: Option<(f64, f64)>) -> Option<bool> {
        // 1. Write Sensors
        if let Some((dist, _angle)) = enemy_info {
            self.vm.grid[0][0] = Value::Int((dist * 10.0) as i64);
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
        Self {
            pattern: MiuraPattern::new(ROWS, COLS),
            rho: 1.0, // Start flat
            target_rho: 1.0,
            enemies: Vec::new(),
            towers: Vec::new(),
            projectiles: Vec::new(),
            resources: 50.0,
            ticks: 0,
            spawn_timer: 0,
            wave: 1,
        }
    }

    pub fn add_tower(&mut self, r: usize, c: usize) {
        // Check bounds
        if r >= self.pattern.rows - 1 || c >= self.pattern.cols - 1 {
            return;
        }
        // Check if occupied
        if self.towers.iter().any(|t| t.r == r && t.c == c) {
            return;
        }

        if self.resources >= 20.0 {
            self.resources -= 20.0;
            self.towers.push(Tower::new(r, c));
        }
    }
}
