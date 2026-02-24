use crate::tree::Tree;
use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use sysinfo::Pid;

#[derive(Clone, Debug)]
pub enum ParasiteState {
    Attached(Pid), // Attached to a process (PID)
    Jumping(Vec2), // Jumping towards a target or just flying (Velocity)
    Falling(Vec2), // Falling due to tree death or failed jump (Velocity)
}

pub struct Parasite {
    pub vm: ChimeraVM,
    pub position: Vec2,
    pub state: ParasiteState,
    pub color: Color,
    pub size: f32,
    pub energy: i64,
}

impl Parasite {
    pub fn new(pos: Vec2) -> Self {
        // Simple DNA: Eat, if energy > threshold, Jump.
        let genes = vec![
            Gene::new(OpCode::Incubate, vec![Nucleotide::from(1)]),
            Gene::new(OpCode::Photosynthesize, vec![]),
        ];

        let strand = Strand { genes };
        let helix = Helix {
            strands: vec![strand],
        };
        let dna = Dna {
            evolution_config: None,
            helix,
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100;

        let mut rng = ::rand::thread_rng();
        Self {
            vm,
            position: pos,
            state: ParasiteState::Falling(vec2(0.0, 0.0)),
            color: Color::from_rgba(rng.gen(), rng.gen(), rng.gen(), 255),
            size: rng.gen_range(3.0..6.0),
            energy: 100,
        }
    }

    pub fn update(&mut self, dt: f32, trees: &[Tree], sun_x: f32) {
        match self.state {
            ParasiteState::Attached(pid) => {
                // Find host
                if let Some(tree) = trees.iter().find(|t| t.stats.pid == pid) {
                    // Update position relative to tree (climb or stick)
                    // For now, stick to a random point on the trunk/branches?
                    // Let's just stick to the main trunk for simplicity or move slightly.
                    // self.position = tree.position + vec2(0.0, -20.0 * tree.scale);

                    // Energy intake from CPU
                    // If sun is shining on this tree (scheduler active), gain energy
                    let dist_to_sun = (tree.position.x - sun_x).abs();
                    let is_lit = dist_to_sun < 50.0; // Approx sun width

                    if is_lit {
                        self.energy += (tree.stats.cpu_usage as i64).max(1);
                        self.vm.energy += 1;
                    }

                    // Metabolism
                    self.energy -= 1;
                    if self.energy <= 0 {
                        self.state = ParasiteState::Falling(vec2(0.0, 0.0));
                    }

                    // VM Execution
                    // Use VM to decide to jump?
                    // For now, random jump if energy is high
                    if self.energy > 200 {
                        let mut rng = ::rand::thread_rng();
                        if rng.gen_bool(0.01) {
                            self.state =
                                ParasiteState::Jumping(vec2(rng.gen_range(-100.0..100.0), -200.0));
                            self.energy -= 50;
                        }
                    }
                } else {
                    // Host died
                    self.state = ParasiteState::Falling(vec2(0.0, 0.0));
                }
            }
            ParasiteState::Jumping(mut vel) | ParasiteState::Falling(mut vel) => {
                // Physics
                vel.y += 400.0 * dt; // Gravity
                self.position += vel * dt;

                // Check collisions
                for tree in trees {
                    if tree.contains(self.position) {
                        self.state = ParasiteState::Attached(tree.stats.pid);
                        return;
                    }
                }

                // Ground collision
                if self.position.y > screen_height() - 20.0 {
                    self.position.y = screen_height() - 20.0;
                    vel.y *= -0.5; // Bounce
                    vel.x *= 0.8; // Friction
                    if vel.length() < 10.0 {
                        // Respawns or just crawls?
                        // Let's make them jump again eventually
                        if rand::gen_range(0, 100) < 5 {
                            vel = vec2(rand::gen_range(-50.0, 50.0), -300.0);
                        }
                    }
                }

                // Update state with new velocity
                if vel.y > 0.0 {
                    self.state = ParasiteState::Falling(vel);
                } else {
                    self.state = ParasiteState::Jumping(vel);
                }
            }
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.size, self.color);
        // Draw energy bar?
        if self.energy > 0 {
            let bar_len = (self.energy as f32 / 10.0).clamp(0.0, 20.0);
            draw_line(
                self.position.x - 10.0,
                self.position.y - 10.0,
                self.position.x - 10.0 + bar_len,
                self.position.y - 10.0,
                2.0,
                GREEN,
            );
        }
    }
}
