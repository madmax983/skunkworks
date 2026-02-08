use crate::physics::{Body, Universe};
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use glam::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::collections::VecDeque;
use std::f32::consts::PI;

pub struct Radar {
    pub angle: f32,
    pub length: f32,
    pub angular_velocity: f32,
}

pub struct Simulation {
    pub universe: Universe,
    pub radar: Radar,
    pub last_executed_vm_index: Option<usize>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            universe: Universe::new(),
            radar: Radar {
                angle: 0.0,
                length: 100.0,
                angular_velocity: 1.0, // rad/s
            },
            last_executed_vm_index: None,
        }
    }

    pub fn spawn_solar_system(&mut self) {
        // Sun
        self.universe.add_body(Body {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            mass: 1000.0,
            radius: 8.0,
            color: Color::Yellow,
            trail: VecDeque::new(),
            vm: None, // Sun has no VM, it is the CPU Clock Source
            name: "Sol".to_string(),
        });

        // Planets
        let mut rng = rand::thread_rng();
        let colors = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Cyan,
            Color::Magenta,
        ];
        let opcodes = [
            OpCode::Photosynthesize,
            OpCode::Incubate,
            OpCode::Drift,
            OpCode::Scramble,
            OpCode::Jump,
        ];

        for i in 0..5 {
            let dist = 30.0 + (i as f32) * 15.0;
            let angle = rng.gen_range(0.0..2.0 * PI);
            let speed = (crate::physics::G * 1000.0 / dist).sqrt();

            let pos = Vec2::new(angle.cos() * dist, angle.sin() * dist);
            let vel = Vec2::new(-angle.sin() * speed, angle.cos() * speed);

            // Create a simple VM for each planet
            // DNA: [Push(1), OpCode, Photosynthesize, Jump(0)]
            let genes = vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: opcodes[i % opcodes.len()].clone(),
                    args: vec![Nucleotide::Number(1)], // Some ops ignore this
                },
                Gene {
                    op: OpCode::Photosynthesize,
                    args: vec![],
                }, // Gain energy
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
            let mut vm = ChimeraVM::new(dna);
            // Give initial energy so they don't die immediately
            vm.energy = 100;

            self.universe.add_body(Body {
                pos,
                vel,
                mass: 10.0 + rng.gen_range(0.0..5.0),
                radius: 3.0,
                color: colors[i % colors.len()],
                trail: VecDeque::new(),
                vm: Some(vm),
                name: format!("Planet-{}", i + 1),
            });
        }
    }

    pub fn step(&mut self, dt: f32) {
        // 1. Physics Step
        self.universe.step(dt);

        // 2. Radar Step
        let old_angle = self.radar.angle;
        self.radar.angle = (self.radar.angle + self.radar.angular_velocity * dt) % (2.0 * PI);
        let new_angle = self.radar.angle;

        // 3. Execution Step
        // Check if any body was crossed by the radar
        // Simplified: Check if body angle is in [old_angle, new_angle]
        // Normalize angles to [0, 2PI)

        let normalize = |a: f32| -> f32 {
            let m = a % (2.0 * PI);
            if m < 0.0 {
                m + 2.0 * PI
            } else {
                m
            }
        };

        let start = normalize(old_angle);
        let end = normalize(new_angle);

        // Handle wrapping
        let crossed = |angle: f32| -> bool {
            let a = normalize(angle);
            if start < end {
                a >= start && a < end
            } else {
                // Wrapped around 0
                a >= start || a < end
            }
        };

        for (i, body) in self.universe.bodies.iter_mut().enumerate() {
            if i == 0 {
                continue;
            } // Skip Sun

            let body_angle = normalize(body.pos.y.atan2(body.pos.x));

            if crossed(body_angle) {
                if let Some(vm) = &mut body.vm {
                    vm.step();
                    // Keep them alive for demo
                    if vm.energy < 50 {
                        vm.energy += 10;
                    }
                    self.last_executed_vm_index = Some(i);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_step() {
        let mut sim = Simulation::new();
        sim.spawn_solar_system();
        sim.step(0.1);
        assert!(sim.universe.bodies.len() > 1);
        // Verify radar moved
        assert!(sim.radar.angle > 0.0);
    }
}
