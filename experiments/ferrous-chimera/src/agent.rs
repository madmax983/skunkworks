use crate::physics::{Body, Vec2};
use chimera_lang::{
    ast::Dna,
    vm::{ChimeraVM, Value},
};
use ratatui::style::Color;

pub struct Agent {
    pub body: Body,
    pub vm: ChimeraVM,
    pub energy: f32,
}

impl Agent {
    pub fn new(dna: Dna, x: f32, y: f32) -> Self {
        // Deterministic color from DNA length or content
        let hue = (dna.helix.strands.len() * 10) as u8;
        let color = Color::Rgb(hue, 255 - hue, 128);

        Self {
            body: Body::new(x, y, 10.0, 2.0, color),
            vm: ChimeraVM::new(dna),
            energy: 100.0,
        }
    }

    pub fn sense(&mut self, magnetism: f32) {
        // Push sensed magnetism to stack (normalized 0-1 mapped to 0-100 int)
        let sensor_val = (magnetism * 100.0) as i64;
        self.vm.stack.push(Value::Int(sensor_val));
    }

    pub fn act(&mut self) -> (Vec2, f32) {
        // Interpret top of stack as action
        // Pop 2 values: X force, Y force.
        // Pop 1 value: Emit amount.

        let emit = if let Some(Value::Int(v)) = self.vm.stack.pop() {
            (v as f32 / 100.0).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let fy = if let Some(Value::Int(v)) = self.vm.stack.pop() {
            (v as f32).clamp(-100.0, 100.0)
        } else {
            0.0
        };

        let fx = if let Some(Value::Int(v)) = self.vm.stack.pop() {
            (v as f32).clamp(-100.0, 100.0)
        } else {
            0.0
        };

        (Vec2::new(fx, fy), emit)
    }
}
