#[cfg(feature = "nova")]
use super::ChimeraVM;
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::Value;
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "nova")]
pub const RETINA_WIDTH: usize = 64;
#[cfg(feature = "nova")]
pub const RETINA_HEIGHT: usize = 32;

#[cfg(feature = "nova")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Retina {
    pub width: usize,
    pub height: usize,
    /// Buffer stores (character, (r, g, b))
    #[allow(clippy::type_complexity)]
    pub buffer: Vec<Vec<(char, (u8, u8, u8))>>,
}

#[cfg(feature = "nova")]
impl Default for Retina {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "nova")]
impl Retina {
    pub fn new() -> Self {
        // Initialize with spaces and black background (effectively empty)
        // We only store FG color for now in tuple.
        // Let's assume (char, fg_color).
        // Default color white?
        let buffer = vec![vec![(' ', (255, 255, 255)); RETINA_WIDTH]; RETINA_HEIGHT];
        Self {
            width: RETINA_WIDTH,
            height: RETINA_HEIGHT,
            buffer,
        }
    }

    pub fn draw(&mut self, y: usize, x: usize, ch: char, r: u8, g: u8, b: u8) {
        if y < self.height && x < self.width {
            self.buffer[y][x] = (ch, (r, g, b));
        }
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = (' ', (r, g, b));
            }
        }
    }
}

#[cfg(feature = "nova")]
pub fn exec_retina_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::RetinaDraw => {
            // Stack: [ ..., packed_color, char_code, y, x ]
            if vm.stack.len() >= 4 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let char_val = vm.stack.pop().unwrap();
                let color_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(c), Value::Int(rgb)) =
                    (x_val, y_val, char_val, color_val)
                {
                    let r = ((rgb >> 16) & 0xFF) as u8;
                    let g = ((rgb >> 8) & 0xFF) as u8;
                    let b = (rgb & 0xFF) as u8;
                    let ch = (c as u8) as char;

                    vm.retina.draw(y as usize, x as usize, ch, r, g, b);
                    vm.energy = vm.energy.saturating_sub(1);
                } else {
                    vm.output
                        .push("Error: Type mismatch for retina_draw".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for retina_draw".to_string());
            }
            None
        }
        OpCode::RetinaClear => {
            // Stack: [ ..., packed_color ]
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(rgb) = val {
                    let r = ((rgb >> 16) & 0xFF) as u8;
                    let g = ((rgb >> 8) & 0xFF) as u8;
                    let b = (rgb & 0xFF) as u8;
                    vm.retina.clear(r, g, b);
                    vm.energy = vm.energy.saturating_sub(10);
                } else {
                    vm.output
                        .push("Error: Type mismatch for retina_clear".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for retina_clear".to_string());
            }
            None
        }
        OpCode::RetinaSize => {
            vm.stack.push(Value::Int(vm.retina.width as i64));
            vm.stack.push(Value::Int(vm.retina.height as i64));
            None
        }
        _ => None,
    }
}
