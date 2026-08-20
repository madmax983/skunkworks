use crate::physics::{Body, Vec2};
use chimera_lang::{
    ast::Dna,
    vm::{ChimeraVM, Value},
};
use ratatui::style::Color;

pub struct Agent {
    pub body: Body,
    pub vm: ChimeraVM,
}

impl Agent {
    fn pop_clamped(vm: &mut ChimeraVM, min: f64, max: f64, scale: f64) -> f64 {
        if let Some(Value::Int(v)) = vm.stack.pop() {
            (v as f64 / scale).clamp(min, max)
        } else {
            0.0
        }
    }
    pub fn new(dna: Dna, x: f64, y: f64) -> Self {
        // Deterministic color from DNA length or content
        let hue = (dna.helix.strands.len() * 10) as u8;
        let color = Color::Rgb(hue, 255 - hue, 128);

        Self {
            body: Body::new(x, y, color),
            vm: ChimeraVM::new(dna),
        }
    }

    pub fn sense(&mut self, magnetism: f64) {
        // Push sensed magnetism to stack (normalized 0-1 mapped to 0-100 int)
        let sensor_val = (magnetism * 100.0) as i64;
        self.vm.stack.push(Value::Int(sensor_val));
    }

    pub fn act(&mut self) -> (Vec2, f64) {
        // Interpret top of stack as action
        // Pop 2 values: X force, Y force.
        // Pop 1 value: Emit amount.

        let emit = Self::pop_clamped(&mut self.vm, 0.0, 1.0, 100.0);
        let fy = Self::pop_clamped(&mut self.vm, -100.0, 100.0, 1.0);
        let fx = Self::pop_clamped(&mut self.vm, -100.0, 100.0, 1.0);

        (Vec2::new(fx, fy), emit)
    }
}
