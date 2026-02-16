#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
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

// --- VM Execution Logic ---

#[cfg(feature = "nova")]
pub fn exec_retina_draw(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 4 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let char_val = vm.stack.pop().unwrap();
        let color_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(c), Value::Int(rgb)) =
            (x_val, y_val, char_val, color_val)
        {
            if x < 0 || y < 0 {
                vm.output.push("Error: Negative coordinates".to_string());
                return None;
            }
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

#[cfg(feature = "nova")]
pub fn exec_retina_clear(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

#[cfg(feature = "nova")]
pub fn exec_retina_size(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.stack.push(Value::Int(vm.retina.width as i64));
    vm.stack.push(Value::Int(vm.retina.height as i64));
    None
}

#[cfg(feature = "nova")]
pub fn exec_scanline(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(y) = val {
            if y >= 0 && (y as usize) < vm.retina.height {
                let row = &vm.retina.buffer[y as usize];
                let mut pixels = Vec::new();
                for (_, (r, g, b)) in row {
                    let rgb = ((*r as i64) << 16) | ((*g as i64) << 8) | (*b as i64);
                    pixels.push(Value::Int(rgb));
                }
                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, pixels));
            } else {
                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, vec![]));
                vm.output
                    .push("Error: Scanline index out of bounds".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for scanline".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for scanline".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_rasterize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 4 {
        let mode_val = vm.stack.pop().unwrap();
        let data_val = vm.stack.pop().unwrap();
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x), Value::Int(mode)) = (y_val, x_val, mode_val) {
            if x < 0 || y < 0 {
                vm.output.push("Error: Negative coordinates".to_string());
                return None;
            }
            let mut pixel_data = Vec::new();

            match data_val {
                Value::Junction(_, vals) => {
                    for v in vals {
                        if let Value::Int(i) = v {
                            pixel_data.push(i);
                        }
                    }
                }
                Value::Int(i) => pixel_data.push(i),
                _ => {}
            }

            match mode {
                3 => {
                    // Sort by brightness?
                    // Approximate brightness: sum of R+G+B
                    pixel_data.sort_by_key(|rgb| {
                        let r = (rgb >> 16) & 0xFF;
                        let g = (rgb >> 8) & 0xFF;
                        let b = rgb & 0xFF;
                        r + g + b
                    });
                }
                _ => {} // Linear, Scatter, XOR don't pre-sort
            }

            let start_x = x as usize;
            let start_y = y as usize;

            for (i, &rgb) in pixel_data.iter().enumerate() {
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;

                let mut draw_x = start_x + i;
                let draw_y = start_y;

                if mode == 1 {
                    // Scatter
                    let mut rng = rand::thread_rng();
                    use rand::Rng;
                    let offset = rng.gen_range(-2..=2);
                    // Use isize for calculation to handle negative result before max
                    draw_x = (draw_x as isize + offset).max(0) as usize;
                }

                if draw_y < vm.retina.height && draw_x < vm.retina.width {
                    if mode == 2 {
                        // XOR
                        let (_, (or, og, ob)) = vm.retina.buffer[draw_y][draw_x];
                        let nr = r ^ or;
                        let ng = g ^ og;
                        let nb = b ^ ob;
                        vm.retina.buffer[draw_y][draw_x] = ('#', (nr, ng, nb));
                    } else {
                        // Default
                        vm.retina.buffer[draw_y][draw_x] = ('#', (r, g, b));
                    }
                }
            }
            vm.energy = vm.energy.saturating_sub(pixel_data.len() as i64 / 10);
        } else {
            vm.output
                .push("Error: Type mismatch for rasterize".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for rasterize".to_string());
    }
    None
}
