use crate::field::MagneticField;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone)]
pub struct Hypha {
    pub pos: Vec2,
    pub vel: Vec2,
    pub vm: ChimeraVM,
    pub magnetism: f32, // 0.0 to 1.0
    pub energy: f32,
    pub age: u32,
    pub id: u64,
}

#[derive(Clone)]
pub struct Stalk {
    pub pos: Vec2,
    pub magnetism: f32,
    pub age: u32,
}

pub enum HyphaAction {
    None,
    Branch(Hypha),
    Die,
}

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl Hypha {
    pub fn new(pos: Vec2, vel: Vec2, dna: Dna) -> Self {
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        Self {
            pos,
            vel,
            vm,
            magnetism: 0.5,
            energy: 100.0,
            age: 0,
            id,
        }
    }

    pub fn random_dna() -> Dna {
        let mut genes = vec![];

        // Simple heuristic genome:
        // 1. GRead at 0,0 (Local B)
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // X
        genes.push(Gene {
            op: OpCode::GRead,
            args: vec![],
        });

        // 2. GRead at 0,1 (Gradient Angle)
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }); // X
        genes.push(Gene {
            op: OpCode::GRead,
            args: vec![],
        });

        // Random Ops
        for _ in 0..10 {
            let op = match macroquad::rand::gen_range(0, 4) {
                0 => OpCode::Add,
                1 => OpCode::Sub,
                2 => OpCode::Mul,
                _ => OpCode::BitXor,
            };
            genes.push(Gene {
                op: OpCode::Dup,
                args: vec![],
            });
            genes.push(Gene { op, args: vec![] });
        }

        // Output Turn (0,3)
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        });
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(3)],
        });
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        });

        // Output Branch (0,5)
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        });
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        });
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        });

        // Loop
        genes.push(Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        });

        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    pub fn update(&mut self, field: &mut MagneticField, dt: f32) -> HyphaAction {
        self.age += 1;
        self.energy -= dt * 2.0;

        // 1. Sense
        let gx = (self.pos.x / screen_width() * field.width as f32) as usize;
        let gy = (self.pos.y / screen_height() * field.height as f32) as usize;
        let local_b = field.get_magnetism(gx, gy);

        // Calculate gradient (simple difference)
        let dx = 1.0;
        let b_right = field.sample(
            self.pos.x / screen_width() * field.width as f32 + dx,
            self.pos.y / screen_height() * field.height as f32,
        );
        let b_down = field.sample(
            self.pos.x / screen_width() * field.width as f32,
            self.pos.y / screen_height() * field.height as f32 + dx,
        );
        let grad_x = b_right - local_b;
        let grad_y = b_down - local_b;
        let grad_len = (grad_x * grad_x + grad_y * grad_y).sqrt();

        // VM Inputs
        // Grid[0][0] = Local B * 100
        self.vm.grid[0][0] = Value::Int((local_b * 100.0) as i64);
        // Grid[0][1] = Gradient Angle * 100
        let angle = grad_y.atan2(grad_x);
        self.vm.grid[0][1] = Value::Int((angle * 100.0) as i64);
        // Grid[0][2] = Self Magnetism * 100
        self.vm.grid[0][2] = Value::Int((self.magnetism * 100.0) as i64);

        // Run VM
        for _ in 0..10 {
            self.vm.step();
        }

        // Outputs
        // Grid[0][3] = Turn Angle (degrees)
        let turn_deg = match self.vm.grid[0][3] {
            Value::Int(v) => (v % 360) as f32,
            _ => 0.0,
        };
        // Grid[0][4] = New Magnetism
        let new_mag_val = match self.vm.grid[0][4] {
            Value::Int(v) => (v.clamp(0, 100) as f32) / 100.0,
            _ => self.magnetism,
        };
        // Grid[0][5] = Branch Probability (0-100)
        let branch_prob = match self.vm.grid[0][5] {
            Value::Int(v) => v.clamp(0, 100) as f32,
            _ => 0.0,
        };

        self.magnetism = self.magnetism * 0.9 + new_mag_val * 0.1;

        // Physics: Magnetotropism
        let target_dir = if grad_len > 0.001 {
            if self.magnetism < 0.5 {
                // Attracted to High B (South)
                vec2(grad_x, grad_y).normalize()
            } else {
                // Attracted to Low B (North)
                -vec2(grad_x, grad_y).normalize()
            }
        } else {
            self.vel.normalize()
        };

        // Combine Genetic Turn + Physical Attraction
        let current_angle = self.vel.y.atan2(self.vel.x);
        let genetic_angle = current_angle + turn_deg.to_radians();
        let genetic_dir = vec2(genetic_angle.cos(), genetic_angle.sin());

        let final_dir = (genetic_dir * 0.5 + target_dir * 0.5).normalize();

        self.vel = final_dir * 50.0; // Constant speed for now

        // Move
        self.pos += self.vel * dt;

        // Modify Field (leave trail)
        let mag_write = (self.magnetism - 0.5) * dt * 0.5; // Scale factor
        field.magnetize(gx, gy, mag_write);

        // Bounds check / Wrap
        if self.pos.x < 0.0 {
            self.pos.x += screen_width();
        }
        if self.pos.x > screen_width() {
            self.pos.x -= screen_width();
        }
        if self.pos.y < 0.0 {
            self.pos.y += screen_height();
        }
        if self.pos.y > screen_height() {
            self.pos.y -= screen_height();
        }

        // Branching
        if macroquad::rand::rand() as f32 % 100.0 < branch_prob && self.energy > 50.0 {
            self.energy -= 20.0;
            // Mutate child
            let mut child = self.clone();
            child.vm.mutate();
            child.energy = 50.0;
            // Branch off at some angle (e.g. 45 deg)
            let rot = Mat2::from_angle(45.0f32.to_radians());
            child.vel = rot * self.vel;

            // Generate new ID
            child.id = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;

            return HyphaAction::Branch(child);
        }

        if self.energy <= 0.0 {
            return HyphaAction::Die;
        }

        HyphaAction::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_uniqueness() {
        let dna = Hypha::random_dna();
        let h1 = Hypha::new(vec2(0.0, 0.0), vec2(0.0, 0.0), dna.clone());
        let h2 = Hypha::new(vec2(0.0, 0.0), vec2(0.0, 0.0), dna.clone());

        assert_ne!(h1.id, h2.id, "IDs should be unique");
        assert!(h2.id > h1.id, "IDs should increment");
    }
}
