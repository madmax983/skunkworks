use chimera_lang::prelude::*;
use crate::lbm::{FluidSim, WIDTH, HEIGHT};
use macroquad::prelude::Vec2;

pub struct Agent {
    pub vm: ChimeraVM,
    pub dna: Dna,
    pub pos: Vec2,
    pub vel: Vec2,
    pub energy: f32,
    pub id: usize,
}

impl Agent {
    pub fn new(id: usize, dna: Dna, pos: Vec2) -> Self {
        Self {
            id,
            vm: ChimeraVM::new(dna.clone()),
            dna,
            pos,
            vel: Vec2::ZERO,
            energy: 100.0,
        }
    }

    pub fn update(&mut self, fluid: &mut FluidSim) {
        // 1. Sense Environment
        // Density at current pos
        let x = self.pos.x as usize;
        let y = self.pos.y as usize;
        // Clamp to ensure we don't panic if float logic goes slightly out of bounds
        let x = x.clamp(0, WIDTH - 1);
        let y = y.clamp(0, HEIGHT - 1);

        let idx = y * WIDTH + x;
        let density = fluid.density[idx];
        let fluid_vx = fluid.velocity_x[idx];
        let fluid_vy = fluid.velocity_y[idx];

        // 2. Run VM
        // Input: [Density, FluidVX, FluidVY, Energy]
        // Push inputs. Note: Stack is LIFO.
        // We push in order so pop order is reverse.
        // If we want [Density, VX, VY, Energy] on stack (Density at bottom), we push Density first.
        self.vm.stack.push(Value::Int((density * 100.0) as i64));
        self.vm.stack.push(Value::Int((fluid_vx * 1000.0) as i64));
        self.vm.stack.push(Value::Int((fluid_vy * 1000.0) as i64));
        self.vm.stack.push(Value::Int(self.energy as i64));

        // Step VM a few times
        for _ in 0..10 {
            self.vm.step();
            if self.vm.halted {
                break;
            }
        }

        // Output: Expect 3 values popped: [Action, AccelX, AccelY]
        // Top of stack is AccelY? It depends on the program.
        // Let's assume the program pushes Action, then AccelX, then AccelY.
        // So we pop AccelY, AccelX, Action.
        let mut accel = Vec2::ZERO;
        let mut action = 0;

        if let Some(Value::Int(ay)) = self.vm.stack.pop() {
            accel.y = (ay as f32 / 100.0).clamp(-1.0, 1.0);
        }
        if let Some(Value::Int(ax)) = self.vm.stack.pop() {
            accel.x = (ax as f32 / 100.0).clamp(-1.0, 1.0);
        }
        if let Some(Value::Int(act)) = self.vm.stack.pop() {
            action = act;
        }

        // 3. Physics
        // Fluid Drag
        let drag = (Vec2::new(fluid_vx, fluid_vy) - self.vel) * 0.5;
        self.vel += drag + accel * 0.1;

        // Limit max velocity
        if self.vel.length() > 2.0 {
            self.vel = self.vel.normalize() * 2.0;
        }

        self.pos += self.vel;

        // Wrap
        if self.pos.x < 0.0 { self.pos.x += WIDTH as f32; }
        if self.pos.x >= WIDTH as f32 { self.pos.x -= WIDTH as f32; }
        if self.pos.y < 0.0 { self.pos.y += HEIGHT as f32; }
        if self.pos.y >= HEIGHT as f32 { self.pos.y -= HEIGHT as f32; }

        // 4. Metabolism
        self.energy -= 0.05; // Base metabolic rate
        self.energy -= accel.length() * 0.05; // Movement cost

        // Action: Eat
        if action == 1 {
            // Eat density
            // Reduce fluid density locally
            // We use add_density with negative value.
            // Ensure we don't create negative density chaos.
            if density > 0.5 {
                fluid.add_density(x, y, -0.05);
                self.energy += 2.0;
            }
        }
    }
}
